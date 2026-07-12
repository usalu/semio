//! ✍️ Writer plugin — declarative writer app bundled as a hot-swappable WASM component.

mod grammar {
// #region grammar
//! ✍️ Lightweight grammar tokenization for writer plugin scenes.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrammarToken {
    pub class: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
struct GrammarRule {
    pattern: regex::Regex,
    class: &'static str,
}

fn jack_rules() -> Vec<GrammarRule> {
    vec![
        GrammarRule {
            pattern: regex::Regex::new(r"(?i)\b(MATCH|WHERE|RETURN|CREATE|DELETE|SET|MERGE|AND|OR)\b").expect("jack keyword"),
            class: "keyword",
        },
        GrammarRule {
            pattern: regex::Regex::new(r#"'[^']*'|"[^"]*""#).expect("jack string"),
            class: "string",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b\d+(?:\.\d+)?\b").expect("jack number"),
            class: "number",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"->|!=|[:=.,\[\]()-]").expect("jack operator"),
            class: "operator",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b[A-Za-z_][A-Za-z0-9_]*\b").expect("jack ident"),
            class: "ident",
        },
    ]
}

fn wire_rules() -> Vec<GrammarRule> {
    vec![
        GrammarRule {
            pattern: regex::Regex::new(r"->").expect("wire keyword"),
            class: "keyword",
        },
        GrammarRule {
            pattern: regex::Regex::new(r#"'[^']*'|"[^"]*""#).expect("wire string"),
            class: "string",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b\d+(?:\.\d+)?\b").expect("wire number"),
            class: "number",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"[:@{}.,\[\]-]").expect("wire operator"),
            class: "operator",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b[A-Za-z_][A-Za-z0-9_.-]*\b").expect("wire ident"),
            class: "ident",
        },
    ]
}

/** @emoji 🎨 Tokenizes source text for a supported writer language id. */
pub fn tokenize_language(text: &str, language_id: &str) -> Vec<GrammarToken> {
    let rules = match language_id {
        "jack" => jack_rules(),
        "wire" => wire_rules(),
        _ => return Vec::new(),
    };
    let mut occupied = vec![false; text.len()];
    let mut tokens = Vec::new();
    for rule in rules {
        for capture in rule.pattern.find_iter(text) {
            let start = capture.start();
            let end = capture.end();
            if occupied[start..end].iter().any(|filled| *filled) {
                continue;
            }
            for slot in &mut occupied[start..end] {
                *slot = true;
            }
            tokens.push(GrammarToken {
                class: rule.class.into(),
                start,
                end,
            });
        }
    }
    tokens.sort_by_key(|token| (token.start, std::cmp::Reverse(token.end)));
    tokens
}
// #endregion grammar
}


use grammar::{tokenize_language, GrammarToken};
use trinity_jack::{complete, example_graph, format as jack_format, lint, semantic_tokens, Diagnostic};
use semio_framework_plugin::{SurfaceKind, PanelGroup, PanelTabSpec,
    build_text_editor_scene, engagement_token_matches, strip_engagement_prefix, tool_button, ui_declarative_sections_to_tree, ui_text, App,
    ActionDescriptor, PluginApp, PluginBundle, TextEditorScene, ToolCategory, ToolNode, UiNode, UiSectionNode,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementInput,
    WindowEngagementOption, WindowMeasure,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, create_default_layout,
};
use semio_framework_plugin::layout::{WindowEngagementPossible, WindowEngagementStatus};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖Constants
const WRITER_PLAY_APP_ID: &str = "writer-play";
const WRITER_PLAY_CONTROLLER_ID: &str = "writer-play";
const WRITER_PLAY_SURFACE_ID: &str = "writer.play";
const WRITER_PLAY_BODY_MAIN: &str = "writer.play.main";
const WRITER_PLAY_BODY_DOCUMENT: &str = "writer.play.document";
const WRITER_PLAY_BODY_CATALOGUE: &str = "writer.play.catalogue";
const WRITER_PLAY_BODY_INSPECTION: &str = "writer.play.inspection";
/// 🌳 Nested children of the document tab — demonstrates the recursive panel-tab tree (stacked tab rows).
const WRITER_PANEL_TAB_DOCUMENT_CONTENT_ID: &str = "framework.panel.document.content";
const WRITER_PANEL_TAB_DOCUMENT_OUTLINE_ID: &str = "framework.panel.document.outline";
const WRITER_PLAY_WINDOW_KIND: &str = "writer-main";
const WRITER_DOCUMENT_SCHEMA: &str = "writer.document";

const JACK_EXAMPLE_JSON: &str = include_str!("../../example/jack.writer.json");
const DAG_JACK_EXAMPLE_JSON: &str = include_str!("../../example/dag.jack.writer.json");
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterCamera {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "default_zoom")]
    zoom: f64,
}

fn default_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterDocument {
    schema: String,
    id: String,
    language_id: String,
    #[serde(default = "default_uri")]
    uri: String,
    #[serde(default)]
    text: String,
    #[serde(default = "default_camera")]
    camera: WriterCamera,
}

fn default_uri() -> String {
    "writer://empty".into()
}

fn default_camera() -> WriterCamera {
    WriterCamera {
        x: 0.0,
        y: 0.0,
        zoom: 1.0,
    }
}

fn empty_writer_document() -> WriterDocument {
    WriterDocument {
        schema: WRITER_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        language_id: "plaintext".into(),
        uri: "writer://empty".into(),
        text: String::new(),
        camera: default_camera(),
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterEditorSelection {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterEditorSettings {
    #[serde(default)]
    show_line_numbers: bool,
    #[serde(default = "default_font_px")]
    font_px: u32,
    #[serde(default = "default_line_height")]
    line_height: u32,
    #[serde(default = "default_tab_size")]
    tab_size: u32,
}

fn default_font_px() -> u32 {
    14
}

/// 🪞 Premigration writer canvas line height (writer/rs `WriterHost::build_scene`).
fn default_line_height() -> u32 {
    22
}

fn default_tab_size() -> u32 {
    2
}

impl Default for WriterEditorSettings {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            font_px: default_font_px(),
            line_height: default_line_height(),
            tab_size: default_tab_size(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterPlayRuntime {
    #[serde(default)]
    selected_ast_ids: Vec<String>,
    #[serde(default)]
    editor_selection: Option<WriterEditorSelection>,
    #[serde(default)]
    format_signal: u32,
    #[serde(default)]
    lint_signal: u32,
    #[serde(default)]
    revision: u32,
    #[serde(default)]
    editor_settings: WriterEditorSettings,
    /// 🐁 AST node id whose tree row is hovered (drives editor hover box + tree highlight).
    #[serde(default)]
    tree_hovered_ast_id: Option<String>,
    /// 🐁 Byte offset last reported as hovered by the editor surface (drives occurrence highlight + tree highlight).
    #[serde(default)]
    editor_hover_offset: Option<usize>,
    #[serde(default)]
    engagement_input: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WriterPlayEnvelope {
    #[serde(flatten)]
    document: WriterDocument,
    #[serde(default)]
    runtime: WriterPlayRuntime,
    #[serde(default)]
    undo_stack: Vec<WriterDocument>,
    #[serde(default)]
    redo_stack: Vec<WriterDocument>,
}

fn parse_envelope(document_json: &str) -> WriterPlayEnvelope {
    if let Ok(envelope) = serde_json::from_str::<WriterPlayEnvelope>(document_json) {
        return envelope;
    }
    let document: WriterDocument = serde_json::from_str(document_json).unwrap_or_else(|_| empty_writer_document());
    WriterPlayEnvelope {
        document,
        runtime: WriterPlayRuntime::default(),
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
    }
}

fn set_document_op(envelope: &WriterPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn push_undo_writer(play: &mut WriterPlayEnvelope) {
    play.undo_stack.push(play.document.clone());
    if play.undo_stack.len() > 32 {
        play.undo_stack.remove(0);
    }
    play.redo_stack.clear();
}
//#endregion 🔖Document

//#region 🔖JackAst
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JackAstNode {
    id: String,
    kind: String,
    label: String,
    start: usize,
    end: usize,
    #[serde(default)]
    children: Vec<JackAstNode>,
}

fn jack_ast_tree_icon(kind: &str) -> Option<&'static str> {
    match kind {
        "query" => Some("file-code"),
        "match" | "create" | "merge" => Some("git-branch"),
        "where" => Some("filter"),
        "return" => Some("corner-down-left"),
        "pattern" | "patternNode" => Some("box"),
        "edge" => Some("arrow-right"),
        "var" => Some("variable"),
        "label" | "property" => Some("tag"),
        "string" => Some("quote"),
        "number" | "bool" | "null" => Some("hash"),
        "error" => Some("alert-circle"),
        _ => None,
    }
}

/// 🌊 Collapses runs of whitespace to a single space, matching premigration `label` derivation.
fn collapse_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
                last_was_space = true;
            }
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}

fn jack_ast_node(kind: &str, start: usize, end: usize, source: &str, children: Vec<JackAstNode>, label: Option<&str>) -> JackAstNode {
    let slice = collapse_whitespace(source.get(start..end).unwrap_or(""));
    let label = label.map(str::to_string).unwrap_or_else(|| if slice.is_empty() { kind.to_string() } else { slice });
    JackAstNode {
        id: format!("jack-ast-{kind}-{start}-{end}"),
        kind: kind.into(),
        label,
        start,
        end,
        children,
    }
}

//#region JackLexer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JackLexKind {
    KwMatch,
    KwWhere,
    KwReturn,
    KwCreate,
    KwDelete,
    KwSet,
    KwMerge,
    KwAnd,
    KwOr,
    Ident,
    Number,
    Str,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Dot,
    Eq,
    Ne,
    Dash,
    Arrow,
    Eof,
}

#[derive(Clone, Debug)]
struct JackLexToken {
    kind: JackLexKind,
    start: usize,
    end: usize,
    text: String,
}

fn tok(kind: JackLexKind, start: usize, end: usize, text: &str) -> JackLexToken {
    JackLexToken { kind, start, end, text: text.to_string() }
}

/// 🔤 Ports premigration `tokenizeJackSource` (writer/core/js/internal.ts) byte-for-byte.
fn tokenize_jack_source(input: &str) -> Vec<JackLexToken> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    while i < len {
        let start = i;
        let c = bytes[i] as char;
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        match c {
            '(' => { tokens.push(tok(JackLexKind::LParen, start, start + 1, "")); i += 1; continue; }
            ')' => { tokens.push(tok(JackLexKind::RParen, start, start + 1, "")); i += 1; continue; }
            '[' => { tokens.push(tok(JackLexKind::LBracket, start, start + 1, "")); i += 1; continue; }
            ']' => { tokens.push(tok(JackLexKind::RBracket, start, start + 1, "")); i += 1; continue; }
            ':' => { tokens.push(tok(JackLexKind::Colon, start, start + 1, "")); i += 1; continue; }
            ',' => { tokens.push(tok(JackLexKind::Comma, start, start + 1, "")); i += 1; continue; }
            '.' => { tokens.push(tok(JackLexKind::Dot, start, start + 1, "")); i += 1; continue; }
            '=' => { tokens.push(tok(JackLexKind::Eq, start, start + 1, "")); i += 1; continue; }
            _ => {}
        }
        if c == '!' && bytes.get(i + 1) == Some(&b'=') {
            tokens.push(tok(JackLexKind::Ne, start, start + 2, ""));
            i += 2;
            continue;
        }
        if c == '-' && bytes.get(i + 1) == Some(&b'>') {
            tokens.push(tok(JackLexKind::Arrow, start, start + 2, ""));
            i += 2;
            continue;
        }
        if c == '-' && i + 1 < len && (bytes[i + 1] as char).is_ascii_digit() {
            let mut j = i + 1;
            while j < len && ((bytes[j] as char).is_ascii_digit() || bytes[j] == b'.') {
                j += 1;
            }
            tokens.push(tok(JackLexKind::Number, start, j, &input[start..j]));
            i = j;
            continue;
        }
        if c == '-' {
            tokens.push(tok(JackLexKind::Dash, start, start + 1, ""));
            i += 1;
            continue;
        }
        if c == '\'' || c == '"' {
            let quote = c;
            i += 1;
            let lit_start = i;
            while i < len && (bytes[i] as char) != quote {
                i += 1;
            }
            let text = input.get(lit_start..i).unwrap_or("").to_string();
            if i < len {
                i += 1;
            }
            tokens.push(tok(JackLexKind::Str, start, i, &text));
            continue;
        }
        if c.is_ascii_digit() {
            let mut j = i;
            while j < len && ((bytes[j] as char).is_ascii_digit() || bytes[j] == b'.') {
                j += 1;
            }
            tokens.push(tok(JackLexKind::Number, start, j, &input[start..j]));
            i = j;
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let mut j = i;
            while j < len && ((bytes[j] as char).is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            let word = &input[start..j];
            let upper = word.to_uppercase();
            let kind = match upper.as_str() {
                "MATCH" => Some(JackLexKind::KwMatch),
                "WHERE" => Some(JackLexKind::KwWhere),
                "RETURN" => Some(JackLexKind::KwReturn),
                "CREATE" => Some(JackLexKind::KwCreate),
                "DELETE" => Some(JackLexKind::KwDelete),
                "SET" => Some(JackLexKind::KwSet),
                "MERGE" => Some(JackLexKind::KwMerge),
                "AND" => Some(JackLexKind::KwAnd),
                "OR" => Some(JackLexKind::KwOr),
                _ => None,
            };
            if let Some(kind) = kind {
                tokens.push(tok(kind, start, j, ""));
            } else {
                tokens.push(tok(JackLexKind::Ident, start, j, word));
            }
            i = j;
            continue;
        }
        i += 1;
    }
    tokens.push(tok(JackLexKind::Eof, len, len, ""));
    tokens
}
//#endregion JackLexer

//#region JackParser
struct JackAstParser<'a> {
    tokens: &'a [JackLexToken],
    source: &'a str,
    pos: usize,
}

impl<'a> JackAstParser<'a> {
    fn new(tokens: &'a [JackLexToken], source: &'a str) -> Self {
        Self { tokens, source, pos: 0 }
    }

    fn peek(&self) -> &JackLexToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn bump(&mut self) -> JackLexToken {
        let token = self.peek().clone();
        if token.kind != JackLexKind::Eof {
            self.pos += 1;
        }
        token
    }

    fn expect_ident(&mut self) -> Result<JackLexToken, String> {
        let token = self.bump();
        if token.kind != JackLexKind::Ident {
            return Err(format!("expected ident at {}", token.start));
        }
        Ok(token)
    }

    fn expect(&mut self, kind: JackLexKind) -> Result<JackLexToken, String> {
        let token = self.bump();
        if token.kind != kind {
            return Err(format!("expected {kind:?} at {}, got {:?}", token.start, token.kind));
        }
        Ok(token)
    }

    fn parse_query(&mut self) -> Result<JackAstNode, String> {
        let mut children = Vec::new();
        while self.peek().kind != JackLexKind::Eof {
            children.push(self.parse_clause()?);
        }
        Ok(jack_ast_node("query", 0, self.source.len(), self.source, children, Some("Query")))
    }

    fn parse_clause(&mut self) -> Result<JackAstNode, String> {
        let start = self.peek().start;
        match self.peek().kind {
            JackLexKind::KwMatch => {
                self.bump();
                let mut patterns = vec![self.parse_pattern()?];
                while self.peek().kind == JackLexKind::Comma {
                    self.bump();
                    patterns.push(self.parse_pattern()?);
                }
                let end = patterns.last().map(|p| p.end).unwrap_or(start);
                Ok(jack_ast_node("match", start, end, self.source, patterns, Some("MATCH")))
            }
            JackLexKind::KwWhere => {
                self.bump();
                let expr = self.parse_expr()?;
                let end = expr.end;
                Ok(jack_ast_node("where", start, end, self.source, vec![expr], Some("WHERE")))
            }
            JackLexKind::KwReturn => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while self.peek().kind == JackLexKind::Comma {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                let end = items.last().map(|i| i.end).unwrap_or(start);
                Ok(jack_ast_node("return", start, end, self.source, items, Some("RETURN")))
            }
            JackLexKind::KwCreate => {
                self.bump();
                let pattern = self.parse_pattern()?;
                let end = pattern.end;
                Ok(jack_ast_node("create", start, end, self.source, vec![pattern], Some("CREATE")))
            }
            JackLexKind::KwDelete => {
                self.bump();
                let mut vars = vec![self.expect_ident()?];
                while self.peek().kind == JackLexKind::Comma {
                    self.bump();
                    vars.push(self.expect_ident()?);
                }
                let end = vars.last().map(|v| v.end).unwrap_or(start);
                let children: Vec<JackAstNode> = vars
                    .iter()
                    .map(|v| jack_ast_node("var", v.start, v.end, self.source, Vec::new(), Some(v.text.as_str())))
                    .collect();
                Ok(jack_ast_node("delete", start, end, self.source, children, Some("DELETE")))
            }
            JackLexKind::KwSet => {
                self.bump();
                let mut items = vec![self.parse_assignment()?];
                while self.peek().kind == JackLexKind::Comma {
                    self.bump();
                    items.push(self.parse_assignment()?);
                }
                let end = items.last().map(|i| i.end).unwrap_or(start);
                Ok(jack_ast_node("set", start, end, self.source, items, Some("SET")))
            }
            JackLexKind::KwMerge => {
                self.bump();
                let pattern = self.parse_pattern()?;
                let end = pattern.end;
                Ok(jack_ast_node("merge", start, end, self.source, vec![pattern], Some("MERGE")))
            }
            _ => Err(format!("unexpected clause at {start}")),
        }
    }

    fn parse_pattern(&mut self) -> Result<JackAstNode, String> {
        let start = self.expect(JackLexKind::LParen)?.start;
        let left = self.parse_pattern_node()?;
        self.expect(JackLexKind::RParen)?;
        if self.peek().kind == JackLexKind::Dash {
            let edge_start = self.bump().start;
            self.expect(JackLexKind::LBracket)?;
            let mut edge_children = Vec::new();
            if self.peek().kind == JackLexKind::Ident {
                let edge_var = self.expect_ident()?;
                edge_children.push(jack_ast_node("edgeVar", edge_var.start, edge_var.end, self.source, Vec::new(), Some(edge_var.text.as_str())));
            }
            if self.peek().kind == JackLexKind::Colon {
                self.bump();
                let edge_kind = self.expect_ident()?;
                edge_children.push(jack_ast_node("edgeKind", edge_kind.start, edge_kind.end, self.source, Vec::new(), Some(edge_kind.text.as_str())));
            }
            self.expect(JackLexKind::RBracket)?;
            self.expect(JackLexKind::Arrow)?;
            self.expect(JackLexKind::LParen)?;
            let right = self.parse_pattern_node()?;
            self.expect(JackLexKind::RParen)?;
            let edge_end = right.end;
            let edge = jack_ast_node("edge", edge_start, edge_end, self.source, edge_children, Some("edge"));
            return Ok(jack_ast_node("pattern", start, edge_end, self.source, vec![left, edge, right], None));
        }
        let end = left.end;
        Ok(jack_ast_node("pattern", start, end, self.source, vec![left], None))
    }

    fn parse_pattern_node(&mut self) -> Result<JackAstNode, String> {
        let start = self.peek().start;
        let var_tok = self.expect_ident()?;
        self.expect(JackLexKind::Colon)?;
        let kind_tok = self.expect_ident()?;
        let var_node = jack_ast_node("var", var_tok.start, var_tok.end, self.source, Vec::new(), Some(var_tok.text.as_str()));
        let kind_node = jack_ast_node("label", kind_tok.start, kind_tok.end, self.source, Vec::new(), Some(kind_tok.text.as_str()));
        let label = format!("{}:{}", var_tok.text, kind_tok.text);
        Ok(jack_ast_node("patternNode", start, kind_tok.end, self.source, vec![var_node, kind_node], Some(label.as_str())))
    }

    fn parse_return_item(&mut self) -> Result<JackAstNode, String> {
        let start = self.peek().start;
        let var_tok = self.expect_ident()?;
        if self.peek().kind == JackLexKind::Dot {
            self.bump();
            let prop_tok = self.expect_ident()?;
            let var_node = jack_ast_node("var", var_tok.start, var_tok.end, self.source, Vec::new(), Some(var_tok.text.as_str()));
            let prop_node = jack_ast_node("property", prop_tok.start, prop_tok.end, self.source, Vec::new(), Some(prop_tok.text.as_str()));
            let label = format!("{}.{}", var_tok.text, prop_tok.text);
            return Ok(jack_ast_node("returnItem", start, prop_tok.end, self.source, vec![var_node, prop_node], Some(label.as_str())));
        }
        let var_node = jack_ast_node("var", var_tok.start, var_tok.end, self.source, Vec::new(), Some(var_tok.text.as_str()));
        Ok(jack_ast_node("returnItem", start, var_tok.end, self.source, vec![var_node], Some(var_tok.text.as_str())))
    }

    fn parse_assignment(&mut self) -> Result<JackAstNode, String> {
        let start = self.peek().start;
        let var_tok = self.expect_ident()?;
        self.expect(JackLexKind::Dot)?;
        let prop_tok = self.expect_ident()?;
        self.expect(JackLexKind::Eq)?;
        let value = self.parse_value()?;
        let var_node = jack_ast_node("var", var_tok.start, var_tok.end, self.source, Vec::new(), Some(var_tok.text.as_str()));
        let prop_node = jack_ast_node("property", prop_tok.start, prop_tok.end, self.source, Vec::new(), Some(prop_tok.text.as_str()));
        let end = value.end;
        Ok(jack_ast_node("assignment", start, end, self.source, vec![var_node, prop_node, value], None))
    }

    fn parse_expr(&mut self) -> Result<JackAstNode, String> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<JackAstNode, String> {
        let mut left = self.parse_and_expr()?;
        while self.peek().kind == JackLexKind::KwOr {
            let op_start = self.bump().start;
            let right = self.parse_and_expr()?;
            let end = right.end;
            left = jack_ast_node("or", op_start, end, self.source, vec![left, right], Some("OR"));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<JackAstNode, String> {
        let mut left = self.parse_cmp_expr()?;
        while self.peek().kind == JackLexKind::KwAnd {
            let op_start = self.bump().start;
            let right = self.parse_cmp_expr()?;
            let end = right.end;
            left = jack_ast_node("and", op_start, end, self.source, vec![left, right], Some("AND"));
        }
        Ok(left)
    }

    fn parse_cmp_expr(&mut self) -> Result<JackAstNode, String> {
        let start = self.peek().start;
        let var_tok = self.expect_ident()?;
        self.expect(JackLexKind::Dot)?;
        let prop_tok = self.expect_ident()?;
        let op = self.bump();
        if op.kind != JackLexKind::Eq && op.kind != JackLexKind::Ne {
            return Err(format!("expected comparison at {}", op.start));
        }
        let value = self.parse_value()?;
        let var_node = jack_ast_node("var", var_tok.start, var_tok.end, self.source, Vec::new(), Some(var_tok.text.as_str()));
        let prop_node = jack_ast_node("property", prop_tok.start, prop_tok.end, self.source, Vec::new(), Some(prop_tok.text.as_str()));
        let kind = if op.kind == JackLexKind::Eq { "eq" } else { "ne" };
        let end = value.end;
        Ok(jack_ast_node(kind, start, end, self.source, vec![var_node, prop_node, value], None))
    }

    fn parse_value(&mut self) -> Result<JackAstNode, String> {
        let token = self.bump();
        match token.kind {
            JackLexKind::Number => Ok(jack_ast_node("number", token.start, token.end, self.source, Vec::new(), Some(token.text.as_str()))),
            JackLexKind::Str => Ok(jack_ast_node("string", token.start, token.end, self.source, Vec::new(), Some(token.text.as_str()))),
            JackLexKind::Ident => {
                let lower = token.text.to_lowercase();
                if lower == "true" || lower == "false" {
                    Ok(jack_ast_node("bool", token.start, token.end, self.source, Vec::new(), Some(token.text.as_str())))
                } else if lower == "null" {
                    Ok(jack_ast_node("null", token.start, token.end, self.source, Vec::new(), Some("null")))
                } else {
                    Err(format!("expected value at {}", token.start))
                }
            }
            _ => Err(format!("expected value at {}", token.start)),
        }
    }
}
//#endregion JackParser

/// 🌳 Parse jack source into a span-tracked AST for hierarchy panels (premigration `parseJackAst`).
fn parse_jack_ast(text: &str) -> JackAstNode {
    let tokens = tokenize_jack_source(text);
    let mut parser = JackAstParser::new(&tokens, text);
    match parser.parse_query() {
        Ok(node) => node,
        Err(message) => jack_ast_node("error", 0, text.len(), text, Vec::new(), Some(message.as_str())),
    }
}

/// 🎯 Deepest AST node containing a byte offset.
fn find_deepest_jack_ast_node_at(root: &JackAstNode, offset: usize) -> Option<&JackAstNode> {
    if offset < root.start || offset >= root.end {
        return None;
    }
    for child in &root.children {
        if let Some(found) = find_deepest_jack_ast_node_at(child, offset) {
            return Some(found);
        }
    }
    Some(root)
}

/// 🔎 Find an AST node by stable id.
fn jack_ast_node_by_id<'a>(root: &'a JackAstNode, id: &str) -> Option<&'a JackAstNode> {
    if root.id == id {
        return Some(root);
    }
    root.children.iter().find_map(|child| jack_ast_node_by_id(child, id))
}

/// 🖱️ Smallest AST node that fully contains a selection range.
fn jack_ast_node_for_selection<'a>(root: &'a JackAstNode, start: usize, end: usize) -> Option<&'a JackAstNode> {
    fn visit<'a>(node: &'a JackAstNode, start: usize, end: usize, best: &mut Option<&'a JackAstNode>) {
        if node.start <= start && node.end >= end {
            let is_better = match best {
                Some(current) => (node.end - node.start) < (current.end - current.start),
                None => true,
            };
            if is_better {
                *best = Some(node);
            }
        }
        for child in &node.children {
            visit(child, start, end, best);
        }
    }
    let mut best = None;
    visit(root, start, end, &mut best);
    best
}

fn jack_ast_to_tree_item(node: &JackAstNode) -> UiTreeItemNode {
    let children: Vec<UiTreeItemNode> = node.children.iter().map(jack_ast_to_tree_item).collect();
    UiTreeItemNode {
        id: node.id.clone(),
        label: node.label.clone(),
        description: Some(node.kind.clone()),
        icon_id: jack_ast_tree_icon(&node.kind).map(str::to_string),
        selected: None,
        default_open: Some(matches!(node.kind.as_str(), "query" | "match" | "pattern" | "return")),
        action: Some(play_action(
            WRITER_PLAY_CONTROLLER_ID,
            "selectAstNode",
            Some(json!({ "id": node.id, "start": node.start, "end": node.end })),
        )),
        hover_action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "setAstHover", Some(json!({ "id": node.id })))),
        unhover_action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "setAstHover", Some(json!({ "id": Value::Null })))),
        actions: None,
        draggable: None,
        drag_data: None,
        items: if children.is_empty() { None } else { Some(children) },
        control: None,
        is_hidden: None,
    }
}
//#endregion 🔖JackAst

//#region 🔖JackEditor
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SelectableSpan {
    start: usize,
    end: usize,
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_end: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tail_start: Option<usize>,
}

/// 🎯 Builds atomic and composite jack spans for token-wise selection (premigration `selectableSpansForJack`).
fn selectable_spans_for_jack(text: &str, tokens: &[GrammarToken]) -> Vec<SelectableSpan> {
    let mut spans: Vec<SelectableSpan> = tokens
        .iter()
        .map(|token| SelectableSpan { start: token.start, end: token.end, kind: "atomic".into(), head_end: None, tail_start: None })
        .collect();
    for i in 0..tokens.len() {
        if i + 2 >= tokens.len() {
            break;
        }
        let head = &tokens[i];
        let colon = &tokens[i + 1];
        let tail = &tokens[i + 2];
        if head.class == "ident" && text.get(colon.start..colon.end) == Some(":") && tail.class == "ident" {
            spans.push(SelectableSpan { start: head.start, end: tail.end, kind: "varLabel".into(), head_end: Some(head.end), tail_start: None });
        }
    }
    for i in 0..tokens.len() {
        let head = &tokens[i];
        if head.class != "ident" {
            continue;
        }
        let mut j = i;
        while j + 2 < tokens.len() && text.get(tokens[j + 1].start..tokens[j + 1].end) == Some(".") && tokens[j + 2].class == "ident" {
            let tail = &tokens[j + 2];
            spans.push(SelectableSpan { start: head.start, end: tail.end, kind: "propertyAccess".into(), head_end: Some(head.end), tail_start: Some(tail.start) });
            j += 2;
        }
    }
    spans.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));
    spans
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JackEditorPlaceholder {
    offset: usize,
    label: String,
}

fn jack_placeholder_visible(caret: usize, offset: usize) -> bool {
    let caret = caret as i64;
    let offset = offset as i64;
    caret >= offset - 32 && caret <= offset + 48
}

fn jack_token_expects_expr(kind: JackLexKind) -> bool {
    matches!(kind, JackLexKind::KwAnd | JackLexKind::KwOr)
}

fn jack_token_expects_pattern(kind: JackLexKind) -> bool {
    matches!(kind, JackLexKind::KwMatch | JackLexKind::KwCreate | JackLexKind::KwMerge)
}

/// 👻 Required jack token placeholders near the caret (premigration `jackEditorPlaceholders`).
fn jack_editor_placeholders(text: &str, caret: usize) -> Vec<JackEditorPlaceholder> {
    let tokens = tokenize_jack_source(text);
    let mut out = Vec::new();
    for i in 0..tokens.len() {
        let token = &tokens[i];
        let next = tokens.get(i + 1);
        let next_kind = next.map(|t| t.kind);
        if jack_token_expects_pattern(token.kind) {
            let bad_next = !matches!(next_kind, Some(JackLexKind::LParen) | Some(JackLexKind::Ident));
            if bad_next {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "(var:Label)".into() });
                }
            }
        }
        if token.kind == JackLexKind::KwReturn {
            let bad = matches!(next_kind, None | Some(JackLexKind::Eof) | Some(JackLexKind::Comma));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "item".into() });
                }
            }
        }
        if token.kind == JackLexKind::KwWhere {
            let bad = matches!(next_kind, None | Some(JackLexKind::Eof) | Some(JackLexKind::KwReturn));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "condition".into() });
                }
            }
        }
        if jack_token_expects_expr(token.kind) {
            let bad = match next_kind {
                None | Some(JackLexKind::Eof) | Some(JackLexKind::KwWhere) | Some(JackLexKind::KwReturn) => true,
                Some(k) => jack_token_expects_expr(k),
            };
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "expr".into() });
                }
            }
        }
        if token.kind == JackLexKind::Colon {
            let bad = matches!(next_kind, None | Some(JackLexKind::Eof) | Some(JackLexKind::RParen) | Some(JackLexKind::Comma) | Some(JackLexKind::RBracket));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "Label".into() });
                }
            }
        }
        if token.kind == JackLexKind::Comma && i > 0 && tokens[i - 1].kind == JackLexKind::KwReturn {
            let bad = matches!(next_kind, None | Some(JackLexKind::Eof) | Some(JackLexKind::Comma));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "item".into() });
                }
            }
        }
        if token.kind == JackLexKind::Dash {
            let bad = !matches!(next_kind, Some(JackLexKind::LBracket));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "[var:Kind]".into() });
                }
            }
        }
        if token.kind == JackLexKind::LBracket {
            let after = tokens.get(i + 1);
            let close = tokens.iter().enumerate().find(|(j, t)| *j > i && t.kind == JackLexKind::RBracket);
            if close.is_none() {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "var:Kind".into() });
                }
            } else {
                let bad = match after {
                    None => true,
                    Some(t) => matches!(t.kind, JackLexKind::RBracket | JackLexKind::Colon),
                };
                if bad {
                    let offset = token.end;
                    if jack_placeholder_visible(caret, offset) {
                        out.push(JackEditorPlaceholder { offset, label: "var:Kind".into() });
                    }
                }
            }
        }
        if token.kind == JackLexKind::Eq || token.kind == JackLexKind::Ne {
            let bad = matches!(next_kind, None | Some(JackLexKind::Eof) | Some(JackLexKind::KwAnd) | Some(JackLexKind::KwOr));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "value".into() });
                }
            }
        }
    }
    out
}

const JACK_NEWLINE_AFTER_KEYWORDS: &[JackLexKind] = &[
    JackLexKind::KwMatch,
    JackLexKind::KwWhere,
    JackLexKind::KwReturn,
    JackLexKind::KwCreate,
    JackLexKind::KwDelete,
    JackLexKind::KwSet,
    JackLexKind::KwMerge,
    JackLexKind::KwAnd,
    JackLexKind::KwOr,
];

fn jack_lex_token_at_offset(tokens: &[JackLexToken], offset: usize) -> Option<&JackLexToken> {
    for token in tokens {
        if token.kind == JackLexKind::Eof {
            break;
        }
        if offset >= token.start && offset <= token.end {
            return Some(token);
        }
    }
    None
}

/// ↩️ Whether a jack query may break onto a new line at a byte offset (premigration `jackNewlineAllowedAt`).
fn jack_newline_allowed_at(text: &str, offset: usize) -> bool {
    let clamped = offset.min(text.len());
    if !text.is_char_boundary(clamped) {
        return false;
    }
    let tokens = tokenize_jack_source(text);
    if let Some(at) = jack_lex_token_at_offset(&tokens, clamped) {
        if clamped > at.start && clamped < at.end {
            return false;
        }
    }
    let before = &text[..clamped];
    let after = &text[clamped..];
    if before.trim_end().chars().last().map(|c| c.is_ascii_alphanumeric() || c == '_').unwrap_or(false) && after.trim_start().starts_with('.') {
        return false;
    }
    if before.trim_end().ends_with(':') && after.trim_start().chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false) {
        return false;
    }
    if before.trim_end().ends_with('.') && after.trim_start().chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false) {
        return false;
    }

    let mut prev: Option<&JackLexToken> = None;
    let mut next: Option<&JackLexToken> = None;
    for token in &tokens {
        if token.kind == JackLexKind::Eof {
            break;
        }
        if token.end <= clamped {
            prev = Some(token);
        }
        if token.start >= clamped && next.is_none() {
            next = Some(token);
            break;
        }
    }

    if before.trim().is_empty() {
        return true;
    }
    if let Some(prev) = prev {
        let gap = &text[prev.end..clamped];
        if !gap.chars().all(|c| c.is_whitespace()) {
            return false;
        }
        if JACK_NEWLINE_AFTER_KEYWORDS.contains(&prev.kind) {
            return true;
        }
        if matches!(prev.kind, JackLexKind::Comma | JackLexKind::RParen | JackLexKind::RBracket | JackLexKind::Arrow) {
            return true;
        }
        if matches!(prev.kind, JackLexKind::Ident | JackLexKind::Number | JackLexKind::Str) {
            return next.map(|n| n.kind != JackLexKind::Dot).unwrap_or(true);
        }
        if matches!(prev.kind, JackLexKind::LParen | JackLexKind::LBracket | JackLexKind::Colon | JackLexKind::Eq | JackLexKind::Ne | JackLexKind::Dash) {
            return true;
        }
    }
    if before.trim().is_empty() && after.trim().is_empty() {
        return true;
    }
    false
}

/// ↩️ All byte offsets at which Enter may insert a newline, for `newlineGatesJson`.
fn jack_newline_gate_offsets(text: &str) -> Vec<usize> {
    (0..=text.len()).filter(|&offset| text.is_char_boundary(offset) && jack_newline_allowed_at(text, offset)).collect()
}

/// 🔗 Bound jack variable names from pattern bindings (premigration `jackBoundVariableNames`).
fn jack_bound_variable_names(text: &str) -> std::collections::HashSet<String> {
    let tokens = tokenize_jack_source(text);
    let mut vars = std::collections::HashSet::new();
    for i in 0..tokens.len() {
        if i + 2 >= tokens.len() {
            break;
        }
        let open = &tokens[i];
        let name = &tokens[i + 1];
        let colon = &tokens[i + 2];
        if matches!(open.kind, JackLexKind::LParen | JackLexKind::LBracket) && name.kind == JackLexKind::Ident && colon.kind == JackLexKind::Colon {
            vars.insert(name.text.clone());
        }
    }
    vars
}

fn is_jack_variable_use_token(tokens: &[JackLexToken], index: usize, bound: &std::collections::HashSet<String>) -> bool {
    let Some(token) = tokens.get(index) else { return false };
    if token.kind != JackLexKind::Ident || !bound.contains(&token.text) {
        return false;
    }
    if index == 0 {
        return true;
    }
    let prev = &tokens[index - 1];
    !matches!(prev.kind, JackLexKind::Colon | JackLexKind::Dot)
}

/// 🔁 All bound-variable occurrences for a jack variable name (premigration `jackVariableOccurrences`).
fn jack_variable_occurrences(text: &str, var_name: &str) -> Vec<(usize, usize)> {
    let tokens = tokenize_jack_source(text);
    let bound = jack_bound_variable_names(text);
    if !bound.contains(var_name) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for i in 0..tokens.len() {
        let token = &tokens[i];
        if token.kind == JackLexKind::Ident && token.text == var_name && is_jack_variable_use_token(&tokens, i, &bound) {
            out.push((token.start, token.end));
        }
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JackSymbolKind {
    Variable,
    Property,
    NodeKind,
    EdgeKind,
}

#[derive(Clone, Debug)]
struct JackSymbolAtCursor {
    kind: JackSymbolKind,
    name: String,
    occurrences: Vec<(usize, usize)>,
}

/// 🎯 Resolve the jack symbol at a byte offset for semantic editor actions (premigration `jackSymbolAtOffset`).
fn jack_symbol_at_offset(text: &str, offset: usize) -> Option<JackSymbolAtCursor> {
    let tokens = tokenize_jack_source(text);
    let clamped = offset.min(text.len());
    let index = tokens.iter().position(|token| token.kind == JackLexKind::Ident && clamped >= token.start && clamped < token.end)?;
    let token = &tokens[index];
    let prev = if index > 0 { tokens.get(index - 1) } else { None };
    if let Some(prev) = prev {
        if prev.kind == JackLexKind::Colon {
            let kind = if index >= 2 && tokens[index - 2].kind == JackLexKind::LBracket { JackSymbolKind::EdgeKind } else { JackSymbolKind::NodeKind };
            return Some(JackSymbolAtCursor { kind, name: token.text.clone(), occurrences: vec![(token.start, token.end)] });
        }
        if prev.kind == JackLexKind::Dot {
            return Some(JackSymbolAtCursor { kind: JackSymbolKind::Property, name: token.text.clone(), occurrences: vec![(token.start, token.end)] });
        }
    }
    let bound = jack_bound_variable_names(text);
    if !is_jack_variable_use_token(&tokens, index, &bound) {
        return None;
    }
    Some(JackSymbolAtCursor { kind: JackSymbolKind::Variable, name: token.text.clone(), occurrences: jack_variable_occurrences(text, &token.text) })
}

/// ✏️ Apply a semantic jack rename across all occurrence spans (premigration `applyJackRename`).
fn apply_jack_rename(text: &str, occurrences: &[(usize, usize)], new_name: &str) -> String {
    let mut sorted: Vec<(usize, usize)> = occurrences.to_vec();
    sorted.sort_by(|a, b| b.0.cmp(&a.0));
    let mut out = text.to_string();
    for (start, end) in sorted {
        if start <= end && end <= out.len() {
            out.replace_range(start..end, new_name);
        }
    }
    out
}

/// 🐁 Resolves tree/editor hover cross-highlighting: (highlighted AST id, tree-hover span, hover occurrences).
fn editor_hover_context(document: &WriterDocument, runtime: &WriterPlayRuntime) -> (Option<String>, Option<(usize, usize)>, Vec<(usize, usize)>) {
    if document.language_id != "jack" {
        return (None, None, Vec::new());
    }
    let root = parse_jack_ast(&document.text);
    let tree_span = runtime.tree_hovered_ast_id.as_ref().and_then(|id| jack_ast_node_by_id(&root, id)).map(|node| (node.start, node.end));
    let editor_hovered_ast_id = runtime.editor_hover_offset.and_then(|offset| find_deepest_jack_ast_node_at(&root, offset)).map(|node| node.id.clone());
    let highlighted = runtime.tree_hovered_ast_id.clone().or(editor_hovered_ast_id);
    let hover_occurrences = runtime
        .editor_hover_offset
        .and_then(|offset| jack_symbol_at_offset(&document.text, offset))
        .filter(|symbol| symbol.kind == JackSymbolKind::Variable)
        .map(|symbol| symbol.occurrences)
        .unwrap_or_default();
    (highlighted, tree_span, hover_occurrences)
}

fn jack_completions_json(text: &str, cursor: usize) -> Option<String> {
    let graph = example_graph();
    let items: Vec<Value> = complete(&graph, text, cursor)
        .into_iter()
        .map(|item| json!({ "label": item.label, "detail": item.detail }))
        .collect();
    serde_json::to_string(&items).ok()
}

/// 🪞 Canonical jack format when possible, else a whitespace-only normalization for other languages.
fn format_writer_text(text: &str, language_id: &str) -> String {
    if language_id == "jack" {
        if let Ok(formatted) = jack_format(text) {
            return formatted;
        }
    }
    let mut normalized: String = text.lines().map(|line| line.trim_end()).collect::<Vec<_>>().join("\n");
    if !text.is_empty() && !normalized.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}
//#endregion 🔖JackEditor

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the writer app; one field per label makes every locale combination compile-checked.
struct WriterLabels {
    document: &'static str,
    empty_query: &'static str,
    language: &'static str,
    jack_description: &'static str,
    camera: &'static str,
    diagnostics: &'static str,
    format: &'static str,
    lint: &'static str,
    line_numbers: &'static str,
    font_size: &'static str,
    line_height: &'static str,
    tab_size: &'static str,
    engagement_placeholder: &'static str,
    editor_mode_status: &'static str,
}

const WRITER_LABELS_NATIVE_EN: WriterLabels = WriterLabels {
    document: "Document",
    empty_query: "(empty query)",
    language: "Language",
    jack_description: "jack — Cypher-inspired trinity query language",
    camera: "Camera",
    diagnostics: "Diagnostics",
    format: "Format",
    lint: "Lint",
    line_numbers: "Line numbers",
    font_size: "Font size",
    line_height: "Line height",
    tab_size: "Tab size",
    engagement_placeholder: "Format, lint, line numbers",
    editor_mode_status: "Text editor",
};

const WRITER_LABELS_NATIVE_DE: WriterLabels = WriterLabels {
    document: "Dokument",
    empty_query: "(leere Abfrage)",
    language: "Sprache",
    jack_description: "jack — von Cypher inspirierte Trinity-Abfragesprache",
    camera: "Kamera",
    diagnostics: "Diagnosen",
    format: "Formatieren",
    lint: "Prüfen",
    line_numbers: "Zeilennummern",
    font_size: "Schriftgröße",
    line_height: "Zeilenhöhe",
    tab_size: "Tabulatorgröße",
    engagement_placeholder: "Format, prüfen, Zeilennummern",
    editor_mode_status: "Texteditor",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown locales fall back to native English.
fn writer_labels(view_state: &ViewState) -> &'static WriterLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &WRITER_LABELS_NATIVE_DE } else { &WRITER_LABELS_NATIVE_EN }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
        args,
    }
}

fn empty_tree_item(id: &str, label: &str) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn render_document_panel(document: &WriterDocument, runtime: &WriterPlayRuntime, labels: &WriterLabels) -> UiNode {
    if document.language_id != "jack" {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "writer-document".into(),
            label: Some(labels.document.into()),
            default_open: Some(true),
            children: vec![ui_text(document.id.clone()), ui_text(document.language_id.clone())],
        }]);
    }
    let root = parse_jack_ast(&document.text);
    let items = if root.kind == "error" {
        vec![UiTreeItemNode {
            description: Some(root.kind.clone()),
            icon_id: jack_ast_tree_icon(&root.kind).map(str::to_string),
            ..empty_tree_item(&root.id, &root.label)
        }]
    } else {
        vec![jack_ast_to_tree_item(&root)]
    };
    let (highlighted_ast_id, _, _) = editor_hover_context(document, runtime);
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "writer-play-document.ast".into(),
            label: Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()),
            default_open: Some(true),
            items: if items.is_empty() { vec![empty_tree_item("writer-play-document.empty", labels.empty_query)] } else { items },
        }],
        selected_ids: Some(runtime.selected_ast_ids.clone()),
        highlighted_ids: highlighted_ast_id.map(|id| vec![id]),
        selection_change: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "setAstSelection", None)),
        drop_action: None,
    })
}

fn render_catalogue_panel(labels: &WriterLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "writer-catalogue".into(),
        label: Some(labels.language.into()),
        default_open: Some(true),
        children: vec![ui_text(labels.jack_description)],
    }])
}

fn render_inspection_panel(document: &WriterDocument, labels: &WriterLabels) -> UiNode {
    let mut sections = vec![
        UiSectionNode {
            id: "writer-inspector.document".into(),
            label: Some(labels.document.into()),
            default_open: Some(true),
            children: vec![
                ui_text(format!("Schema: {}", document.schema)),
                ui_text(format!("Id: {}", document.id)),
                ui_text(format!("Language: {}", document.language_id)),
                ui_text(format!("Uri: {}", document.uri)),
                ui_text(format!("Lines: {}", document.text.lines().count())),
            ],
        },
        UiSectionNode {
            id: "writer-inspector.camera".into(),
            label: Some(labels.camera.into()),
            default_open: Some(false),
            children: vec![
                ui_text(format!("x: {}", document.camera.x)),
                ui_text(format!("y: {}", document.camera.y)),
                ui_text(format!("zoom: {}", document.camera.zoom)),
            ],
        },
    ];
    if document.language_id == "jack" {
        let graph = example_graph();
        let messages: Vec<String> = lint(&graph, &document.text).into_iter().map(|diag: Diagnostic| diag.message).take(8).collect();
        if !messages.is_empty() {
            sections.push(UiSectionNode {
                id: "writer-inspector.diagnostics".into(),
                label: Some(labels.diagnostics.into()),
                default_open: Some(true),
                children: messages.into_iter().map(ui_text).collect(),
            });
        }
    }
    ui_declarative_sections_to_tree(&sections)
}
//#endregion 🔖Panels

//#region 🔖Scene
fn render_main_scene(document: &WriterDocument, runtime: &WriterPlayRuntime) -> UiNode {
    let is_jack = document.language_id == "jack";
    let selection = runtime.editor_selection.clone().unwrap_or_default();
    let cursor = selection.end;
    let selection_json = runtime.editor_selection.as_ref().map(|s| json!({ "start": s.start, "end": s.end }).to_string());

    let grammar_tokens = tokenize_language(&document.text, &document.language_id);
    let tokens_json = if is_jack {
        serde_json::to_string(&semantic_tokens(&document.text)).ok()
    } else {
        serde_json::to_string(&grammar_tokens).ok()
    };

    let diagnostics_json = if is_jack {
        let graph = example_graph();
        let diagnostics: Vec<Value> = lint(&graph, &document.text)
            .into_iter()
            .map(|diag: Diagnostic| json!({ "start": diag.start, "end": diag.end, "severity": diag.severity, "message": diag.message }))
            .collect();
        Some(serde_json::to_string(&diagnostics).unwrap_or_else(|_| "[]".into()))
    } else if runtime.lint_signal > 0 {
        Some(json!([{ "start": 0, "end": document.text.len().max(1), "severity": "info", "message": format!("Lint pass #{}", runtime.lint_signal) }]).to_string())
    } else {
        None
    };

    let selectable_spans_json = is_jack.then(|| serde_json::to_string(&selectable_spans_for_jack(&document.text, &grammar_tokens)).unwrap_or_else(|_| "[]".into()));
    let placeholders_json = is_jack.then(|| serde_json::to_string(&jack_editor_placeholders(&document.text, cursor)).unwrap_or_else(|_| "[]".into()));
    let newline_gates_json = is_jack.then(|| serde_json::to_string(&jack_newline_gate_offsets(&document.text)).unwrap_or_else(|_| "[]".into()));

    let (_, tree_hover_span, hover_occurrences) = editor_hover_context(document, runtime);
    let hover_json = Some(match tree_hover_span {
        Some((start, end)) => json!({ "start": start, "end": end }).to_string(),
        None => "null".to_string(),
    });

    let caret_symbol = if is_jack && selection.start == selection.end { jack_symbol_at_offset(&document.text, selection.start) } else { None };
    let (selection_occurrences, rename_json): (Vec<(usize, usize)>, Option<String>) = match &caret_symbol {
        Some(symbol) if symbol.kind == JackSymbolKind::Variable => {
            let occurrences_json: Vec<Value> = symbol.occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
            let rename = json!({ "name": symbol.name, "occurrences": occurrences_json }).to_string();
            (symbol.occurrences.clone(), Some(rename))
        }
        _ => (Vec::new(), None),
    };

    let occurrences_json = is_jack.then(|| {
        let hover: Vec<Value> = hover_occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
        let selection: Vec<Value> = selection_occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
        json!({
            "hover": serde_json::to_string(&hover).unwrap_or_else(|_| "[]".into()),
            "selection": serde_json::to_string(&selection).unwrap_or_else(|_| "[]".into()),
        })
        .to_string()
    });

    let extra_carets_json = (!selection_occurrences.is_empty())
        .then(|| serde_json::to_string(&selection_occurrences.iter().map(|(s, _)| *s).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into()));

    let completions_json = is_jack.then(|| jack_completions_json(&document.text, cursor)).flatten();

    build_text_editor_scene(
        WRITER_PLAY_SURFACE_ID,
        WRITER_PLAY_CONTROLLER_ID,
        TextEditorScene {
            buffer: document.text.clone(),
            language: Some(document.language_id.clone()),
            selection_json,
            tokens_json,
            diagnostics_json,
            completions_json,
            occurrences_json,
            overlays_json: runtime.editor_settings.show_line_numbers.then(|| json!({ "lineNumbers": true }).to_string()),
            placeholders_json,
            extra_carets_json,
            selectable_spans_json,
            settings_json: Some(serde_json::to_string(&runtime.editor_settings).unwrap_or_else(|_| "{}".into())),
            camera_json: Some(json!({ "x": document.camera.x, "y": document.camera.y, "zoom": document.camera.zoom }).to_string()),
            hover_json,
            newline_gates_json,
            rename_json,
        },
    )
}
//#endregion 🔖Scene

//#region 🔖Engagement
/// 💬 Natural-language engagement parsing (premigration `applyEngagement`). Accepts both the
/// spaced form (wgpu REPL) and the React shell's PascalCased, separator-stripped drafts (e.g.
/// `"Font16"`, `"LineNumbers"` — see `strip_engagement_prefix`).
fn apply_engagement(play: &mut WriterPlayEnvelope, value: &str) {
    let trimmed = value.trim();
    play.runtime.engagement_input.clear();
    play.runtime.revision += 1;
    if trimmed.is_empty() {
        return;
    }
    if engagement_token_matches(trimmed, "format") {
        let formatted = format_writer_text(&play.document.text, &play.document.language_id);
        if formatted != play.document.text {
            push_undo_writer(play);
            play.document.text = formatted;
        }
        play.runtime.format_signal += 1;
        return;
    }
    if engagement_token_matches(trimmed, "lint") {
        play.runtime.lint_signal += 1;
        return;
    }
    if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
        play.runtime.editor_settings.show_line_numbers = !play.runtime.editor_settings.show_line_numbers;
        return;
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
        if let Ok(px) = rest.parse::<u32>() {
            play.runtime.editor_settings.font_px = px;
        }
        return;
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
        if let Ok(size) = rest.parse::<u32>() {
            play.runtime.editor_settings.tab_size = size.max(1);
        }
    }
}
//#endregion 🔖Engagement

//#region 🔖WriterApp
struct WriterApp;

impl PluginApp for WriterApp {
    fn app_id(&self) -> &str {
        WRITER_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&WriterPlayEnvelope {
            document: empty_writer_document(),
            runtime: WriterPlayRuntime::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
        .expect("writer document json")
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        match action {
            "textEdit" | "setDocument" => {
                if let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) {
                    push_undo_writer(&mut play);
                    play.document.text = text.into();
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<WriterPlayEnvelope>(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                    if let Ok(parsed) = serde_json::from_value::<WriterDocument>(next.clone()) {
                        play.document = parsed;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setDocumentJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(parsed) = serde_json::from_str(json_text) {
                        push_undo_writer(&mut play);
                        play.document = parsed;
                        play.runtime.revision += 1;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "setText" => {
                if let Some(text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) {
                    push_undo_writer(&mut play);
                    play.document.text = text.into();
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        play.document.camera = parsed;
                        play.runtime.revision += 1;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "formatDocument" => {
                let formatted = format_writer_text(&play.document.text, &play.document.language_id);
                if formatted != play.document.text {
                    push_undo_writer(&mut play);
                    play.document.text = formatted;
                }
                play.runtime.format_signal += 1;
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "requestCompletions" => {
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "commitRename" => {
                let Some(new_text) = args.and_then(|value| value.get("text")).and_then(|value| value.as_str()) else {
                    return Vec::new();
                };
                let occurrences: Option<Vec<(usize, usize)>> = args
                    .and_then(|value| value.get("occurrences"))
                    .and_then(|value| value.as_array())
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(|item| {
                                let start = item.get("start")?.as_u64()? as usize;
                                let end = item.get("end")?.as_u64()? as usize;
                                Some((start, end))
                            })
                            .collect::<Vec<_>>()
                    })
                    .filter(|items| !items.is_empty());
                if let Some(occurrences) = occurrences {
                    push_undo_writer(&mut play);
                    play.document.text = apply_jack_rename(&play.document.text, &occurrences, new_text);
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
                if let (Some(start), Some(end)) = (
                    args.and_then(|value| value.get("start")).and_then(|value| value.as_u64()),
                    args.and_then(|value| value.get("end")).and_then(|value| value.as_u64()),
                ) {
                    let start = start as usize;
                    let end = end as usize;
                    if start <= end && end <= play.document.text.len() {
                        push_undo_writer(&mut play);
                        let mut next = play.document.text.clone();
                        next.replace_range(start..end, new_text);
                        play.document.text = next;
                        play.runtime.revision += 1;
                        return vec![set_document_op(&play)];
                    }
                }
            }
            "lintDocument" => {
                play.runtime.lint_signal += 1;
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "textSelect" | "setEditorSelection" => {
                let start = args.and_then(|value| value.get("start")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                let end = args.and_then(|value| value.get("end")).and_then(|value| value.as_u64()).unwrap_or(start as u64) as usize;
                play.runtime.editor_selection = Some(WriterEditorSelection { start, end });
                if play.document.language_id == "jack" {
                    let root = parse_jack_ast(&play.document.text);
                    play.runtime.selected_ast_ids = jack_ast_node_for_selection(&root, start.min(end), start.max(end))
                        .map(|node| vec![node.id.clone()])
                        .unwrap_or_default();
                } else {
                    play.runtime.selected_ast_ids.clear();
                }
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "selectAstNode" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
                let start = args.and_then(|value| value.get("start")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                let end = args.and_then(|value| value.get("end")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                play.runtime.selected_ast_ids = if id.is_empty() { Vec::new() } else { vec![id.into()] };
                play.runtime.editor_selection = Some(WriterEditorSelection { start, end });
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "setAstSelection" => {
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| value.as_array())
                    .map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                play.runtime.selected_ast_ids = ids.clone();
                if let Some(id) = ids.first() {
                    if play.document.language_id == "jack" {
                        let root = parse_jack_ast(&play.document.text);
                        play.runtime.editor_selection = jack_ast_node_by_id(&root, id).map(|node| WriterEditorSelection { start: node.start, end: node.end });
                    }
                }
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "setAstHover" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                if id != play.runtime.tree_hovered_ast_id {
                    play.runtime.tree_hovered_ast_id = id;
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            "textHover" => {
                let start = args.and_then(|value| value.get("start")).and_then(|value| value.as_u64()).map(|v| v as usize);
                let end = args.and_then(|value| value.get("end")).and_then(|value| value.as_u64()).map(|v| v as usize);
                let offset = match (start, end) {
                    (Some(s), Some(e)) => Some(s + e.saturating_sub(s) / 2),
                    _ => None,
                };
                if offset != play.runtime.editor_hover_offset {
                    play.runtime.editor_hover_offset = offset;
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            "toggleLineNumbers" => {
                play.runtime.editor_settings.show_line_numbers = !play.runtime.editor_settings.show_line_numbers;
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "setEditorSetting" => {
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                match field {
                    "fontPx" => {
                        if let Some(px) = value.and_then(|v| v.as_u64()) {
                            play.runtime.editor_settings.font_px = px as u32;
                        }
                    }
                    "lineHeight" => {
                        if let Some(px) = value.and_then(|v| v.as_u64()) {
                            play.runtime.editor_settings.line_height = px as u32;
                        }
                    }
                    "tabSize" => {
                        if let Some(px) = value.and_then(|v| v.as_u64()) {
                            play.runtime.editor_settings.tab_size = px.max(1) as u32;
                        }
                    }
                    _ => return Vec::new(),
                }
                play.runtime.revision += 1;
                return vec![set_document_op(&play)];
            }
            "engagementInput" => {
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                if value != play.runtime.engagement_input {
                    play.runtime.engagement_input = value;
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            "engagementSubmit" => {
                let value = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| play.runtime.engagement_input.clone());
                apply_engagement(&mut play, &value);
                return vec![set_document_op(&play)];
            }
            "undo" => {
                if let Some(previous) = play.undo_stack.pop() {
                    play.redo_stack.push(play.document.clone());
                    play.document = previous;
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            "redo" => {
                if let Some(next) = play.redo_stack.pop() {
                    play.undo_stack.push(play.document.clone());
                    play.document = next;
                    play.runtime.revision += 1;
                    return vec![set_document_op(&play)];
                }
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("empty");
                let document = match example_id {
                    "jack" => serde_json::from_str::<WriterDocument>(JACK_EXAMPLE_JSON).unwrap_or_else(|_| empty_writer_document()),
                    "dag.jack" => serde_json::from_str::<WriterDocument>(DAG_JACK_EXAMPLE_JSON).unwrap_or_else(|_| empty_writer_document()),
                    _ => empty_writer_document(),
                };
                return vec![set_document_op(&WriterPlayEnvelope {
                    document,
                    runtime: WriterPlayRuntime::default(),
                    undo_stack: Vec::new(),
                    redo_stack: Vec::new(),
                })];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let labels = writer_labels(view_state);
        match body_key {
            WRITER_PLAY_BODY_MAIN => render_main_scene(&play.document, &play.runtime),
            WRITER_PLAY_BODY_DOCUMENT => render_document_panel(&play.document, &play.runtime, labels),
            WRITER_PLAY_BODY_CATALOGUE => render_catalogue_panel(labels),
            WRITER_PLAY_BODY_INSPECTION => render_inspection_panel(&play.document, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn tools(&self, _document_json: &str, view_state: &ViewState) -> Vec<ToolNode> {
        let labels = writer_labels(view_state);
        vec![
            tool_button("writer-format", "align-left", labels.format, play_action(WRITER_PLAY_CONTROLLER_ID, "formatDocument", None)).with_category(ToolCategory::Actions),
            tool_button("writer-lint", "alert-circle", labels.lint, play_action(WRITER_PLAY_CONTROLLER_ID, "lintDocument", None)).with_category(ToolCategory::Actions),
        ]
    }

    fn window_engagements(&self, document_json: &str, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let play = parse_envelope(document_json);
        let settings = &play.runtime.editor_settings;
        let labels = writer_labels(view_state);
        let engagement = WindowEngagement {
            session_active: Some(false),
            options: Some(vec![WindowEngagementOption {
                id: "writer-line-numbers".into(),
                label: Some(labels.line_numbers.into()),
                icon_id: Some("list-ordered".into()),
                pressed: Some(settings.show_line_numbers),
                disabled: None,
                action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "toggleLineNumbers", None)),
            }]),
            input: Some(WindowEngagementInput {
                id: Some("writer-engagement-input".into()),
                value: Some(play.runtime.engagement_input.clone()),
                placeholder: Some(labels.engagement_placeholder.into()),
                disabled: None,
                on_change: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "engagementInput", None)),
                on_submit: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "engagementSubmit", None)),
                on_repeat_last: None,
                on_abort: None,
            }),
            control: None,
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "writer-editor-mode".into(), text: labels.editor_mode_status.into() }]),
            possible_engagements: Some(vec![
                WindowEngagementPossible { id: "writer-format".into(), label: labels.format.into(), detail: None, action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "formatDocument", None)) },
                WindowEngagementPossible { id: "writer-lint".into(), label: labels.lint.into(), detail: None, action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "lintDocument", None)) },
                WindowEngagementPossible { id: "writer-line-numbers".into(), label: labels.line_numbers.into(), detail: None, action: Some(play_action(WRITER_PLAY_CONTROLLER_ID, "toggleLineNumbers", None)) },
            ]),
        };
        HashMap::from([(WRITER_PLAY_WINDOW_KIND.to_string(), engagement)])
    }

    fn window_measures(&self, document_json: &str, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let play = parse_envelope(document_json);
        let settings = &play.runtime.editor_settings;
        let labels = writer_labels(view_state);
        let measures = vec![
            WindowMeasure::Slider {
                id: "writer-font-size-measure".into(),
                label: Some(labels.font_size.into()),
                value: settings.font_px as f64,
                min: 10.0,
                max: 24.0,
                step: Some(1.0),
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "setEditorSetting", Some(json!({ "field": "fontPx" }))),
            },
            WindowMeasure::Slider {
                id: "writer-line-height-measure".into(),
                label: Some(labels.line_height.into()),
                value: settings.line_height as f64,
                min: 16.0,
                max: 40.0,
                step: Some(1.0),
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "setEditorSetting", Some(json!({ "field": "lineHeight" }))),
            },
            WindowMeasure::Slider {
                id: "writer-tab-size-measure".into(),
                label: Some(labels.tab_size.into()),
                value: settings.tab_size as f64,
                min: 1.0,
                max: 8.0,
                step: Some(1.0),
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "setEditorSetting", Some(json!({ "field": "tabSize" }))),
            },
            WindowMeasure::Toggle {
                id: "writer-line-numbers-measure".into(),
                icon_id: "list-ordered".into(),
                label: Some(labels.line_numbers.into()),
                pressed: settings.show_line_numbers,
                text: None,
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "toggleLineNumbers", None),
            },
        ];
        HashMap::from([(WRITER_PLAY_WINDOW_KIND.to_string(), measures)])
    }
}
//#endregion 🔖WriterApp

//#region 🔖Manifest
fn create_writer_app() -> App {
    App::from_builder(
        App::builder(WRITER_PLAY_APP_ID, "Writer").document(["semio", "writer"])
            .icon_id("writer")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(WRITER_PLAY_WINDOW_KIND, "Jack", WRITER_PLAY_BODY_MAIN, SurfaceKind::TextEditor)
            .default_layout(create_default_layout(
                &[WRITER_PLAY_WINDOW_KIND.into()],
                "row",
                Some(&[100.0]),
                Some(&["Jack".into()]),
            ))
            .panel_tab_tree(PanelTabSpec::group(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                vec![
                    PanelTabSpec::leaf(WRITER_PANEL_TAB_DOCUMENT_CONTENT_ID, "Content", PanelGroup::Workbench, WRITER_PLAY_BODY_DOCUMENT),
                    PanelTabSpec::leaf(WRITER_PANEL_TAB_DOCUMENT_OUTLINE_ID, "Outline", PanelGroup::Workbench, WRITER_PLAY_BODY_DOCUMENT),
                ],
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                WRITER_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                WRITER_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("empty", "Empty", serde_json::to_string(&empty_writer_document()).unwrap())
    .example("jack", "Jack", JACK_EXAMPLE_JSON)
    .example("dag.jack", "Dag Jack", DAG_JACK_EXAMPLE_JSON)
    .program("writer", "Writer", "text.document")
}

fn writer_bundle() -> PluginBundle {
    PluginBundle::new("writer", "Writer", "0.1.0").register_app(create_writer_app(), || Box::new(WriterApp))
}

semio_framework_plugin::plugin_exports!(writer_bundle);
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_text_editor_scene() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let node = app.render(WRITER_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn renders_document_tree_for_jack() {
        let app = WriterApp;
        let document = JACK_EXAMPLE_JSON.to_string();
        let node = app.render(WRITER_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Query"));
    }

    #[test]
    fn renders_catalogue_panel() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let node = app.render(WRITER_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("jack"));
    }

    #[test]
    fn format_document_reformats_jack_query() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&WriterPlayEnvelope {
            document: WriterDocument {
                schema: WRITER_DOCUMENT_SCHEMA.into(),
                id: "jack".into(),
                language_id: "jack".into(),
                uri: "writer://jack".into(),
                text: "MATCH (a:Piece)   WHERE a.name='core' RETURN a.name".into(),
                camera: default_camera(),
            },
            runtime: WriterPlayRuntime::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
        .unwrap();
        let ops = app.handle_action_patch_ops("formatDocument", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let envelope: WriterPlayEnvelope = serde_json::from_str(
            &serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string(),
        )
        .unwrap();
        assert!(envelope.document.text.contains('\n'));
        assert_eq!(envelope.runtime.format_signal, 1);
    }

    #[test]
    fn jack_completions_use_example_fixture() {
        let json = jack_completions_json("RETURN a.", 9).unwrap_or_default();
        assert!(!json.is_empty());
    }

    #[test]
    fn set_text_action_updates_document() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let ops = app.handle_action_patch_ops(
            "setText",
            Some(&json!({ "text": "MATCH (a) RETURN a" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        assert!(ops[0].contains("MATCH"));
    }

    const CANONICAL_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";

    #[test]
    fn parses_full_jack_ast_shape() {
        let root = parse_jack_ast(CANONICAL_QUERY);
        assert_eq!(root.kind, "query");
        assert_eq!(root.children.len(), 3);
        assert_eq!(root.children[0].kind, "match");
        assert_eq!(root.children[1].kind, "where");
        assert_eq!(root.children[2].kind, "return");
        let pattern = &root.children[0].children[0];
        assert_eq!(pattern.kind, "pattern");
        assert_eq!(pattern.children.len(), 3);
        assert_eq!(pattern.children[1].kind, "edge");
    }

    #[test]
    fn selection_maps_to_smallest_containing_ast_node() {
        let root = parse_jack_ast(CANONICAL_QUERY);
        let a_offset = CANONICAL_QUERY.find("a:Piece").unwrap();
        let node = jack_ast_node_for_selection(&root, a_offset, a_offset + 1).expect("node");
        assert_eq!(node.kind, "var");
    }

    #[test]
    fn selectable_spans_include_var_label_and_property_access() {
        let text = "MATCH (a1:Piece) RETURN a1.name";
        let tokens = tokenize_language(text, "jack");
        let spans = selectable_spans_for_jack(text, &tokens);
        assert!(spans.iter().any(|s| s.kind == "varLabel" && s.start == 7 && s.end == 15));
        assert!(spans.iter().any(|s| s.kind == "propertyAccess" && s.start == 24 && s.end == 31));
    }

    #[test]
    fn symbol_occurrences_find_bound_variable_uses() {
        let symbol = jack_symbol_at_offset(CANONICAL_QUERY, CANONICAL_QUERY.find("a.name").unwrap()).expect("symbol");
        assert_eq!(symbol.kind, JackSymbolKind::Variable);
        assert_eq!(symbol.occurrences.len(), 3);
    }

    #[test]
    fn symbol_at_label_position_is_node_kind_not_variable() {
        let symbol = jack_symbol_at_offset(CANONICAL_QUERY, CANONICAL_QUERY.find("Piece").unwrap() + 1).expect("symbol");
        assert_eq!(symbol.kind, JackSymbolKind::NodeKind);
    }

    #[test]
    fn placeholders_suggest_expr_after_and() {
        let text = "MATCH (a:Piece) WHERE a.name = 'x' AND ";
        let placeholders = jack_editor_placeholders(text, text.len());
        assert!(placeholders.iter().any(|p| p.label == "expr"));
    }

    #[test]
    fn placeholders_suggest_label_after_colon() {
        let text = "MATCH (a:";
        let placeholders = jack_editor_placeholders(text, text.len());
        assert!(placeholders.iter().any(|p| p.label == "Label"));
    }

    #[test]
    fn newline_gates_allow_after_match_close_paren() {
        let text = "MATCH (a:Piece)";
        let gates = jack_newline_gate_offsets(text);
        assert!(gates.contains(&text.len()));
    }

    #[test]
    fn newline_gates_disallow_inside_token() {
        let text = "MATCH (a:Piece)";
        let inside = text.find("Piece").unwrap() + 2;
        assert!(!jack_newline_allowed_at(text, inside));
    }

    #[test]
    fn newline_gates_disallow_before_dot() {
        let text = "RETURN a.name";
        let before_dot = text.find('.').unwrap();
        assert!(!jack_newline_allowed_at(text, before_dot));
    }

    #[test]
    fn commit_rename_with_occurrences_renames_all_spans() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&WriterPlayEnvelope {
            document: WriterDocument {
                schema: WRITER_DOCUMENT_SCHEMA.into(),
                id: "jack".into(),
                language_id: "jack".into(),
                uri: "writer://jack".into(),
                text: CANONICAL_QUERY.into(),
                camera: default_camera(),
            },
            runtime: WriterPlayRuntime::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
        .unwrap();
        let occurrences = jack_variable_occurrences(CANONICAL_QUERY, "a");
        assert_eq!(occurrences.len(), 3);
        let occurrences_json: Vec<Value> = occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
        let ops = app.handle_action_patch_ops(
            "commitRename",
            Some(&json!({ "occurrences": occurrences_json, "text": "piece" })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let envelope: WriterPlayEnvelope = serde_json::from_str(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string()).unwrap();
        assert_eq!(envelope.document.text.matches("piece").count(), 3);
        assert_eq!(envelope.document.text.matches("a:Piece").count(), 0);
    }

    #[test]
    fn engagement_submit_parses_font_size() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let ops = app.handle_action_patch_ops("engagementSubmit", Some(&json!({ "value": "font 16" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let envelope: WriterPlayEnvelope = serde_json::from_str(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string()).unwrap();
        assert_eq!(envelope.runtime.editor_settings.font_px, 16);
        assert_eq!(envelope.runtime.engagement_input, "");
    }

    #[test]
    fn engagement_submit_parses_normalized_shell_drafts() {
        // The React shell PascalCases and strips separators from every draft before submitting it
        // (`normalizeEngagementActionText`), so "font 16" arrives as "Font16", "tab 4" as "Tab4",
        // and "line numbers" as "LineNumbers".
        let mut app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();

        let ops = app.handle_action_patch_ops("engagementSubmit", Some(&json!({ "value": "Font16" })), &document, &ViewState::default());
        let envelope: WriterPlayEnvelope = serde_json::from_str(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string()).unwrap();
        assert_eq!(envelope.runtime.editor_settings.font_px, 16);

        let ops = app.handle_action_patch_ops("engagementSubmit", Some(&json!({ "value": "Tab4" })), &document, &ViewState::default());
        let envelope: WriterPlayEnvelope = serde_json::from_str(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string()).unwrap();
        assert_eq!(envelope.runtime.editor_settings.tab_size, 4);

        let before_toggle: WriterPlayEnvelope = serde_json::from_str(&document).unwrap();
        let ops = app.handle_action_patch_ops("engagementSubmit", Some(&json!({ "value": "LineNumbers" })), &document, &ViewState::default());
        let envelope: WriterPlayEnvelope = serde_json::from_str(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string()).unwrap();
        assert_eq!(envelope.runtime.editor_settings.show_line_numbers, !before_toggle.runtime.editor_settings.show_line_numbers);
    }

    #[test]
    fn window_measures_expose_font_line_height_tab_and_toggle() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let measures = app.window_measures(&document, &ViewState::default());
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert_eq!(main.len(), 4);
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Toggle { id, .. } if id == "writer-line-numbers-measure")));
    }

    #[test]
    fn window_engagements_expose_format_lint_placeholder() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let engagements = app.window_engagements(&document, &ViewState::default());
        let main = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("main engagement");
        let placeholder = main.input.as_ref().and_then(|i| i.placeholder.as_ref()).expect("placeholder");
        assert!(placeholder.contains("Format"));
        assert_eq!(main.possible_engagements.as_ref().map(|v| v.len()), Some(3));
    }

    #[test]
    fn tools_include_format_and_lint_buttons() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let tools = app.tools(&document, &ViewState::default());
        let json = serde_json::to_string(&tools).unwrap();
        assert!(json.contains("writer-format"));
        assert!(json.contains("writer-lint"));
    }

    #[test]
    fn scene_emits_placeholders_selectable_spans_and_newline_gates_for_jack() {
        let app = WriterApp;
        let document = JACK_EXAMPLE_JSON.to_string();
        let node = app.render(WRITER_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("placeholdersJson"));
        assert!(json.contains("selectableSpansJson"));
        assert!(json.contains("newlineGatesJson"));
    }

    #[test]
    fn set_ast_hover_updates_tree_highlight_and_scene_hover() {
        let mut app = WriterApp;
        let document = JACK_EXAMPLE_JSON.to_string();
        let root = parse_jack_ast(&parse_envelope(&document).document.text);
        let ops = app.handle_action_patch_ops("setAstHover", Some(&json!({ "id": root.id })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let next_document = serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string();
        let tree_node = app.render(WRITER_PLAY_BODY_DOCUMENT, &next_document, &ViewState::default());
        let tree_json = serde_json::to_string(&tree_node).unwrap();
        assert!(tree_json.contains(&root.id));
        let scene_node = app.render(WRITER_PLAY_BODY_MAIN, &next_document, &ViewState::default());
        let scene_value = serde_json::to_value(&scene_node).unwrap();
        let hover_json = scene_value["textEditor"]["hoverJson"].as_str().expect("hoverJson string");
        let hover_range: Value = serde_json::from_str(hover_json).unwrap();
        assert_eq!(hover_range["start"].as_u64(), Some(root.start as u64));
        assert_eq!(hover_range["end"].as_u64(), Some(root.end as u64));
    }

    #[test]
    fn manifest_includes_dag_jack_example() {
        let bundle = writer_bundle();
        let manifest = &bundle.manifest;
        assert!(manifest.apps.iter().any(|a| a.id == WRITER_PLAY_APP_ID));
        assert!(manifest.examples.iter().any(|e| e.id == "dag.jack" && e.app_id == WRITER_PLAY_APP_ID));
    }

    #[test]
    fn set_active_example_loads_jack_fixture() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let ops = app.handle_action_patch_ops("setActiveExample", Some(&json!({ "exampleId": "jack" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let envelope: WriterPlayEnvelope = serde_json::from_str(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string()).unwrap();
        assert_eq!(envelope.document.id, "jack");
        assert!(envelope.document.text.contains("MATCH"));
    }

    #[test]
    fn set_active_example_loads_dag_jack_fixture() {
        let mut app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let ops = app.handle_action_patch_ops("setActiveExample", Some(&json!({ "exampleId": "dag.jack" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let envelope: WriterPlayEnvelope = serde_json::from_str(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string()).unwrap();
        assert_eq!(envelope.document.id, "dag-jack");
    }

    #[test]
    fn set_active_example_falls_back_to_empty_document() {
        let mut app = WriterApp;
        let document = JACK_EXAMPLE_JSON.to_string();
        let ops = app.handle_action_patch_ops("setActiveExample", Some(&json!({ "exampleId": "empty" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let envelope: WriterPlayEnvelope = serde_json::from_str(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].to_string()).unwrap();
        assert_eq!(envelope.document.id, "empty");
        assert_eq!(envelope.document.text, "");
    }

    #[test]
    fn writer_labels_resolve_native_by_default() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, &document, &ViewState::default());
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("\"Document\""));
        assert!(inspection_json.contains("\"Camera\""));
        let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("\"Language\""));
        assert!(catalogue_json.contains("Cypher-inspired"));
        let tools = app.tools(&document, &ViewState::default());
        let tools_json = serde_json::to_string(&tools).unwrap();
        assert!(tools_json.contains("\"category\":\"actions\""));
        assert!(tools_json.contains("\"Format\""));
        assert!(tools_json.contains("\"Lint\""));
        let measures = app.window_measures(&document, &ViewState::default());
        let measures_json = serde_json::to_string(&measures).unwrap();
        assert!(measures_json.contains("Font size"));
        assert!(measures_json.contains("Line numbers"));
        assert!(!measures_json.contains("Schriftgröße"));
    }

    #[test]
    fn writer_labels_resolve_german_locale() {
        let app = WriterApp;
        let document = serde_json::to_string(&empty_writer_document()).unwrap();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, &document, &view_state);
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Dokument"));
        assert!(inspection_json.contains("Kamera"));
        assert!(!inspection_json.contains("\"Camera\""));
        let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, &document, &view_state);
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Sprache"));
        let tools = app.tools(&document, &view_state);
        let tools_json = serde_json::to_string(&tools).unwrap();
        assert!(tools_json.contains("Formatieren"));
        assert!(tools_json.contains("Prüfen"));
        let measures = app.window_measures(&document, &view_state);
        let measures_json = serde_json::to_string(&measures).unwrap();
        assert!(measures_json.contains("Schriftgröße"));
        assert!(measures_json.contains("Zeilennummern"));
        let engagements = app.window_engagements(&document, &view_state);
        let engagements_json = serde_json::to_string(&engagements).unwrap();
        assert!(engagements_json.contains("Texteditor"));
    }
}
//#endregion 🧪Tests
