use std::collections::HashMap;

use tree_sitter_highlight::{Highlighter, HtmlRenderer};

pub use tree_sitter_highlight::HighlightConfiguration;

pub const NAMES: &[&str] = &[
    "annotation",
    "attribute",
    "boolean",
    "character",
    "character.special",
    "comment",
    "conditional",
    "constant",
    "constant.builtin",
    "constant.macro",
    "constructor",
    "debug",
    "define",
    "error",
    "exception",
    "field",
    "float",
    "function",
    "function.call",
    "function.builtin",
    "function.macro",
    "include",
    "keyword",
    "keyword.function",
    "keyword.operator",
    "keyword.return",
    "label",
    "method",
    "method.call",
    "namespace",
    "none",
    "number",
    "operator",
    "parameter",
    "parameter.reference",
    "preproc",
    "property",
    "punctuation.delimiter",
    "punctuation.bracket",
    "punctuation.special",
    "repeat",
    "storageclass",
    "string",
    "string.regex",
    "string.escape",
    "string.special",
    "symbol",
    "tag",
    "tag.attribute",
    "tag.delimiter",
    "text",
    "text.strong",
    "text.emphasis",
    "text.underline",
    "text.strike",
    "text.title",
    "text.literal",
    "text.uri",
    "text.math",
    "text.reference",
    "text.environment",
    "text.environment.name",
    "text.note",
    "text.warning",
    "text.danger",
    "todo",
    "type",
    "type.builtin",
    "type.qualifier",
    "type.definition",
    "variable",
    "variable.builtin",
];

/// Languages
#[derive(Default)]
pub struct Languages<'a> {
    inner: HashMap<&'a str, (HighlightConfiguration, Vec<String>)>,
}

impl<'a> Languages<'a> {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub fn insert(&mut self, lang: &'a str, config: HighlightConfiguration) -> &mut Self {
        self.insert_with_names(lang, config, NAMES)
    }

    pub fn insert_with_names(
        &mut self,
        lang: &'a str,
        mut config: HighlightConfiguration,
        names: &[&str],
    ) -> &mut Self {
        config.configure(names);
        self.inner.insert(lang, (config, names_to_classes(names)));
        self
    }

    pub fn get<'b>(&'a self, lang: &'b str) -> Option<&'a (HighlightConfiguration, Vec<String>)> {
        self.inner.get(lang)
    }

    pub fn render(&self, lang: &str, source: &[u8]) -> Option<String> {
        if let Some((config, names)) = self.get(lang) {
            let mut highlighter = Highlighter::new();
            if let Ok(highlights) = highlighter.highlight(config, source, None, |_| None) {
                let mut renderer = HtmlRenderer::new();
                return renderer
                    .render(highlights, source, &|h| {
                        names.get(h.0).map(String::as_bytes).unwrap_or(b"")
                    })
                    .map(|_| String::new())
                    .map(|mut s| {
                        s.push_str("<pre class=language-");
                        s.push_str(lang);
                        s.push_str("><code>");
                        renderer.lines().for_each(|line| {
                            s.push_str("<span class=line>");
                            s.push_str(line);
                            s.push_str("</span>");
                        });
                        s.push_str("</code></pre>");
                        s
                    })
                    .ok();
            };
        }

        None
    }
}

fn names_to_classes(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .map(|n| {
            let mut s = String::new();
            s.push_str("class=");
            s.push_str(n);
            s
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn highlighting() -> Result<(), Box<dyn Error>> {
        let mut languages = Languages::new();
        languages.insert(
            "rust",
            HighlightConfiguration::new(
                tree_sitter_rust::language(),
                include_str!("../queries/rust/highlights.scm"),
                include_str!("../queries/rust/injections.scm"),
                include_str!("../queries/rust/locals.scm"),
            )?,
        );

        assert_eq!(
            languages
                .render(
                    "rust",
                    r#"""
use std::net::SocketAddr;
use viz::{Request, Result, Router, Server, ServiceMaker};

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello Viz")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let app = Router::new().get("/", index);

    if let Err(err) = Server::bind(&addr)
        .serve(ServiceMaker::from(app))
        .await
    {
        println!("{}", err);
    }

    Ok(())
}
        """#
                    .as_bytes()
                )
                .unwrap(),
            "<pre class=language-rust><code><span class=line><span class=string>&quot;&quot;</span>\n</span><span class=line><span class=include>use</span> <span class=variable>std</span><span class=punctuation.delimiter>::</span><span class=variable>net</span><span class=punctuation.delimiter>::</span><span class=variable>SocketAddr</span><span class=punctuation.delimiter>;</span>\n</span><span class=line><span class=include>use</span> <span class=variable>viz</span><span class=punctuation.delimiter>::</span><span class=punctuation.bracket>{</span><span class=variable>Request</span><span class=punctuation.delimiter>,</span> <span class=variable>Result</span><span class=punctuation.delimiter>,</span> <span class=variable>Router</span><span class=punctuation.delimiter>,</span> <span class=variable>Server</span><span class=punctuation.delimiter>,</span> <span class=variable>ServiceMaker</span><span class=punctuation.bracket>}</span><span class=punctuation.delimiter>;</span>\n</span><span class=line>\n</span><span class=line><span class=keyword>async</span> <span class=keyword.function>fn</span> <span class=variable>index</span><span class=punctuation.bracket>(</span>_<span class=punctuation.delimiter>:</span> <span class=type>Request</span><span class=punctuation.bracket>)</span> <span class=operator>-&gt;</span> <span class=type>Result</span><span class=operator>&lt;</span><span class=operator>&amp;</span><span class=label>&#39;</span><span class=variable>static</span> <span class=type.builtin>str</span><span class=operator>&gt;</span> <span class=punctuation.bracket>{</span>\n</span><span class=line>    <span class=variable>Ok</span><span class=punctuation.bracket>(</span><span class=string>&quot;Hello Viz&quot;</span><span class=punctuation.bracket>)</span>\n</span><span class=line><span class=punctuation.bracket>}</span>\n</span><span class=line>\n</span><span class=line><span class=punctuation.special>#</span><span class=punctuation.bracket>[</span><span class=variable>tokio</span><span class=punctuation.delimiter>::</span><span class=variable>main</span><span class=punctuation.bracket>]</span>\n</span><span class=line><span class=keyword>async</span> <span class=keyword.function>fn</span> <span class=variable>main</span><span class=punctuation.bracket>(</span><span class=punctuation.bracket>)</span> <span class=operator>-&gt;</span> <span class=type>Result</span><span class=operator>&lt;</span><span class=punctuation.bracket>(</span><span class=punctuation.bracket>)</span><span class=operator>&gt;</span> <span class=punctuation.bracket>{</span>\n</span><span class=line>    <span class=keyword>let</span> <span class=variable>addr</span> <span class=operator>=</span> <span class=variable>SocketAddr</span><span class=punctuation.delimiter>::</span><span class=variable>from</span><span class=punctuation.bracket>(</span><span class=punctuation.bracket>(</span><span class=punctuation.bracket>[</span><span class=number>127</span><span class=punctuation.delimiter>,</span> <span class=number>0</span><span class=punctuation.delimiter>,</span> <span class=number>0</span><span class=punctuation.delimiter>,</span> <span class=number>1</span><span class=punctuation.bracket>]</span><span class=punctuation.delimiter>,</span> <span class=number>3000</span><span class=punctuation.bracket>)</span><span class=punctuation.bracket>)</span><span class=punctuation.delimiter>;</span>\n</span><span class=line>    <span class=variable>println</span><span class=operator>!</span><span class=punctuation.bracket>(</span><span class=string>&quot;listening on {}&quot;</span>, <span class=variable>addr</span><span class=punctuation.bracket>)</span><span class=punctuation.delimiter>;</span>\n</span><span class=line>\n</span><span class=line>    <span class=keyword>let</span> <span class=variable>app</span> <span class=operator>=</span> <span class=variable>Router</span><span class=punctuation.delimiter>::</span><span class=variable>new</span><span class=punctuation.bracket>(</span><span class=punctuation.bracket>)</span><span class=punctuation.delimiter>.</span><span class=field>get</span><span class=punctuation.bracket>(</span><span class=string>&quot;/&quot;</span><span class=punctuation.delimiter>,</span> <span class=variable>index</span><span class=punctuation.bracket>)</span><span class=punctuation.delimiter>;</span>\n</span><span class=line>\n</span><span class=line>    <span class=conditional>if</span> <span class=keyword>let</span> <span class=variable>Err</span><span class=punctuation.bracket>(</span><span class=variable>err</span><span class=punctuation.bracket>)</span> <span class=operator>=</span> <span class=variable>Server</span><span class=punctuation.delimiter>::</span><span class=variable>bind</span><span class=punctuation.bracket>(</span><span class=operator>&amp;</span><span class=variable>addr</span><span class=punctuation.bracket>)</span>\n</span><span class=line>        <span class=punctuation.delimiter>.</span><span class=field>serve</span><span class=punctuation.bracket>(</span><span class=variable>ServiceMaker</span><span class=punctuation.delimiter>::</span><span class=variable>from</span><span class=punctuation.bracket>(</span><span class=variable>app</span><span class=punctuation.bracket>)</span><span class=punctuation.bracket>)</span>\n</span><span class=line>        <span class=punctuation.delimiter>.</span><span class=keyword>await</span>\n</span><span class=line>    <span class=punctuation.bracket>{</span>\n</span><span class=line>        <span class=variable>println</span><span class=operator>!</span><span class=punctuation.bracket>(</span><span class=string>&quot;{}&quot;</span>, <span class=variable>err</span><span class=punctuation.bracket>)</span><span class=punctuation.delimiter>;</span>\n</span><span class=line>    <span class=punctuation.bracket>}</span>\n</span><span class=line>\n</span><span class=line>    <span class=variable>Ok</span><span class=punctuation.bracket>(</span><span class=punctuation.bracket>(</span><span class=punctuation.bracket>)</span><span class=punctuation.bracket>)</span>\n</span><span class=line><span class=punctuation.bracket>}</span>\n</span><span class=line>        <span class=string>&quot;&quot;</span><span class=punctuation.delimiter></span>\n</span></code></pre>"
        );

        Ok(())
    }
}
