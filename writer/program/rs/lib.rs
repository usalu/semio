//! ✍️ Writer program — declarative writer app bundled as a hot-swappable WASM program.

mod grammar {
//#region 🔖Grammar
//! ✍️ Lightweight grammar tokenization for writer program scenes.

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
//#endregion 🔖Grammar
}


use grammar::{tokenize_language, GrammarToken};
use trinity_jack::{complete, example_graph, format as jack_format, lint, semantic_tokens, Diagnostic};
use writer::{empty_writer_projection, WriterCamera, WriterOperation, WriterProjection};
use semio_framework_program::{SurfaceKind, PanelGroup, PanelTabSpec,
    build_text_editor_scene, engagement_token_matches, is_de_locale, localized_label_map, resolve_labels, strip_engagement_prefix,
    tree_item, ui_declarative_sections_to_tree, ui_text, App,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, ActionDescriptor, ActionEmit, AppLabelsOverlay, AppLabelsOverlayExt,
    DocumentApp, DocumentView, IconName, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelTreeBuilder, ArtifactKindSpec, TextEditorScene, UiNode, UiPresence, UiSectionNode,
    UiTreeItemNode, ViewState, WindowEngagement, WindowEngagementInput,
    WindowEngagementOption, WindowMeasure,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, create_default_layout,
};
use semio_framework_program::{WindowEngagementPossible, WindowEngagementStatus};
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

/// 📄 The `jack` example document, handcrafted in the `.writer` DSL (see `store::DocumentDsl`) instead
/// of JSON — {@link jack_example_document}/{@link jack_example_json} are the only ways it should be
/// consumed.
const JACK_EXAMPLE_TEXT: &str = include_str!("../../example/jack.writer");
/// 📄 The `dag.jack` example document, handcrafted in the `.writer` DSL — see {@link JACK_EXAMPLE_TEXT}.
const DAG_JACK_EXAMPLE_TEXT: &str = include_str!("../../example/dag.jack.writer");
//#endregion 🔖Constants

//#region 🔖Examples
/// 📄 The `jack` example, parsed once from {@link JACK_EXAMPLE_TEXT} — the source of truth for every
/// call site below (`setActiveExample`, `.example("jack", ...)`, tests); never re-embed the raw text.
fn jack_example_document() -> WriterProjection {
    <WriterProjection as store::DocumentDsl>::parse_dsl(JACK_EXAMPLE_TEXT).unwrap_or_else(|_| empty_writer_projection())
}

/// 📄 JSON re-serialization of {@link jack_example_document}, for the framework-generic call sites
/// (`.example(...)`, `render(...)`) that still take a document as a JSON string.
fn jack_example_json() -> String {
    serde_json::to_string(&jack_example_document()).expect("serialize jack example document")
}

/// 📄 The `dag.jack` example, parsed once from {@link DAG_JACK_EXAMPLE_TEXT} — see {@link jack_example_document}.
fn dag_jack_example_document() -> WriterProjection {
    <WriterProjection as store::DocumentDsl>::parse_dsl(DAG_JACK_EXAMPLE_TEXT).unwrap_or_else(|_| empty_writer_projection())
}

/// 📄 JSON re-serialization of {@link dag_jack_example_document} — see {@link jack_example_json}.
fn dag_jack_example_json() -> String {
    serde_json::to_string(&dag_jack_example_document()).expect("serialize dag.jack example document")
}
//#endregion 🔖Examples

//#region 🔖Types
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
//#endregion 🔖Types

//#region 🔖DocumentHelpers
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

/// 🌉 Adapts trinity_jack's shared [`trinity_jack::SpannedNode`] tree into writer's own [`JackAstNode`]
/// (adds the stable tree-item `id` the outline panel needs; `kind`/`label`/spans pass through unchanged).
fn jack_ast_from_spanned(node: &trinity_jack::SpannedNode) -> JackAstNode {
    JackAstNode {
        id: format!("jack-ast-{}-{}-{}", node.kind, node.start, node.end),
        kind: node.kind.clone(),
        label: node.label.clone(),
        start: node.start,
        end: node.end,
        children: node.children.iter().map(jack_ast_from_spanned).collect(),
    }
}

/// 🌳 Parse jack source into a span-tracked AST for hierarchy panels, via the shared `trinity_jack` parser.
fn parse_jack_ast(text: &str) -> JackAstNode {
    jack_ast_from_spanned(&trinity_jack::parse_spanned(text))
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
        // 🛟 `and_then(IconName::from_str)` (not the panicking `IconName::from`) so a jack AST kind
        // whose icon string isn't (yet) in the shared icon catalog just renders with no icon.
        icon_id: jack_ast_tree_icon(&node.kind).and_then(IconName::from_str),
        presence: UiPresence::default(),
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
        dimmed: None,
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

/// 🔤 Fine-grained, never-fails jack tokens for editor heuristics — routed through `trinity_jack`'s shared
/// forgiving lexer instead of a hand-rolled writer copy.
fn jack_tokens(text: &str) -> Vec<trinity_jack::SpannedToken> {
    trinity_jack::lex_spanned(text, true).unwrap_or_default()
}

fn jack_token_expects_expr(token: &trinity_jack::Token) -> bool {
    matches!(token, trinity_jack::Token::And | trinity_jack::Token::Or)
}

fn jack_token_expects_pattern(token: &trinity_jack::Token) -> bool {
    matches!(token, trinity_jack::Token::KwMatch | trinity_jack::Token::KwCreate | trinity_jack::Token::KwMerge)
}

/// 👻 Required jack token placeholders near the caret (premigration `jackEditorPlaceholders`).
fn jack_editor_placeholders(text: &str, caret: usize) -> Vec<JackEditorPlaceholder> {
    use trinity_jack::Token;
    let tokens = jack_tokens(text);
    let mut out = Vec::new();
    for i in 0..tokens.len() {
        let token = &tokens[i];
        let next = tokens.get(i + 1);
        let next_kind = next.map(|t| &t.token);
        if jack_token_expects_pattern(&token.token) {
            let bad_next = !matches!(next_kind, Some(Token::LParen) | Some(Token::Ident(_)));
            if bad_next {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "(var:Label)".into() });
                }
            }
        }
        if token.token == Token::KwReturn {
            let bad = matches!(next_kind, None | Some(Token::Eof) | Some(Token::Comma));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "item".into() });
                }
            }
        }
        if token.token == Token::KwWhere {
            let bad = matches!(next_kind, None | Some(Token::Eof) | Some(Token::KwReturn));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "condition".into() });
                }
            }
        }
        if jack_token_expects_expr(&token.token) {
            let bad = match next_kind {
                None | Some(Token::Eof) | Some(Token::KwWhere) | Some(Token::KwReturn) => true,
                Some(k) => jack_token_expects_expr(k),
            };
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "expr".into() });
                }
            }
        }
        if token.token == Token::Colon {
            let bad = matches!(next_kind, None | Some(Token::Eof) | Some(Token::RParen) | Some(Token::Comma) | Some(Token::RBracket));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "Label".into() });
                }
            }
        }
        if token.token == Token::Comma && i > 0 && tokens[i - 1].token == Token::KwReturn {
            let bad = matches!(next_kind, None | Some(Token::Eof) | Some(Token::Comma));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "item".into() });
                }
            }
        }
        if token.token == Token::Dash {
            let bad = !matches!(next_kind, Some(Token::LBracket));
            if bad {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "[var:Kind]".into() });
                }
            }
        }
        if token.token == Token::LBracket {
            let after = tokens.get(i + 1);
            let close = tokens.iter().enumerate().find(|(j, t)| *j > i && t.token == Token::RBracket);
            if close.is_none() {
                let offset = token.end;
                if jack_placeholder_visible(caret, offset) {
                    out.push(JackEditorPlaceholder { offset, label: "var:Kind".into() });
                }
            } else {
                let bad = match after {
                    None => true,
                    Some(t) => matches!(t.token, Token::RBracket | Token::Colon),
                };
                if bad {
                    let offset = token.end;
                    if jack_placeholder_visible(caret, offset) {
                        out.push(JackEditorPlaceholder { offset, label: "var:Kind".into() });
                    }
                }
            }
        }
        if token.token == Token::Eq || token.token == Token::Ne {
            let bad = matches!(next_kind, None | Some(Token::Eof) | Some(Token::And) | Some(Token::Or));
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

const JACK_NEWLINE_AFTER_KEYWORDS: &[trinity_jack::Token] = &[
    trinity_jack::Token::KwMatch,
    trinity_jack::Token::KwWhere,
    trinity_jack::Token::KwReturn,
    trinity_jack::Token::KwCreate,
    trinity_jack::Token::KwDelete,
    trinity_jack::Token::KwSet,
    trinity_jack::Token::KwMerge,
    trinity_jack::Token::And,
    trinity_jack::Token::Or,
];

fn jack_lex_token_at_offset(tokens: &[trinity_jack::SpannedToken], offset: usize) -> Option<&trinity_jack::SpannedToken> {
    for token in tokens {
        if token.token == trinity_jack::Token::Eof {
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
    use trinity_jack::Token;
    let clamped = offset.min(text.len());
    if !text.is_char_boundary(clamped) {
        return false;
    }
    let tokens = jack_tokens(text);
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

    let mut prev: Option<&trinity_jack::SpannedToken> = None;
    let mut next: Option<&trinity_jack::SpannedToken> = None;
    for token in &tokens {
        if token.token == Token::Eof {
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
        if JACK_NEWLINE_AFTER_KEYWORDS.contains(&prev.token) {
            return true;
        }
        if matches!(prev.token, Token::Comma | Token::RParen | Token::RBracket | Token::Arrow) {
            return true;
        }
        if matches!(prev.token, Token::Ident(_) | Token::Number(_) | Token::StringLit(_)) {
            return next.map(|n| n.token != Token::Dot).unwrap_or(true);
        }
        if matches!(prev.token, Token::LParen | Token::LBracket | Token::Colon | Token::Eq | Token::Ne | Token::Dash) {
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
    use trinity_jack::Token;
    let tokens = jack_tokens(text);
    let mut vars = std::collections::HashSet::new();
    for i in 0..tokens.len() {
        if i + 2 >= tokens.len() {
            break;
        }
        let open = &tokens[i];
        let name = &tokens[i + 1];
        let colon = &tokens[i + 2];
        if matches!(open.token, Token::LParen | Token::LBracket) && colon.token == Token::Colon {
            if let Token::Ident(text) = &name.token {
                vars.insert(text.clone());
            }
        }
    }
    vars
}

fn is_jack_variable_use_token(tokens: &[trinity_jack::SpannedToken], index: usize, bound: &std::collections::HashSet<String>) -> bool {
    use trinity_jack::Token;
    let Some(token) = tokens.get(index) else { return false };
    let Token::Ident(text) = &token.token else { return false };
    if !bound.contains(text) {
        return false;
    }
    if index == 0 {
        return true;
    }
    let prev = &tokens[index - 1];
    !matches!(prev.token, Token::Colon | Token::Dot)
}

/// 🔁 All bound-variable occurrences for a jack variable name (premigration `jackVariableOccurrences`).
fn jack_variable_occurrences(text: &str, var_name: &str) -> Vec<(usize, usize)> {
    use trinity_jack::Token;
    let tokens = jack_tokens(text);
    let bound = jack_bound_variable_names(text);
    if !bound.contains(var_name) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for i in 0..tokens.len() {
        let token = &tokens[i];
        if matches!(&token.token, Token::Ident(text) if text == var_name) && is_jack_variable_use_token(&tokens, i, &bound) {
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
    use trinity_jack::Token;
    let tokens = jack_tokens(text);
    let clamped = offset.min(text.len());
    let index = tokens.iter().position(|token| matches!(token.token, Token::Ident(_)) && clamped >= token.start && clamped < token.end)?;
    let token = &tokens[index];
    let Token::Ident(name) = &token.token else { return None };
    let prev = if index > 0 { tokens.get(index - 1) } else { None };
    if let Some(prev) = prev {
        if prev.token == Token::Colon {
            let kind = if index >= 2 && tokens[index - 2].token == Token::LBracket { JackSymbolKind::EdgeKind } else { JackSymbolKind::NodeKind };
            return Some(JackSymbolAtCursor { kind, name: name.clone(), occurrences: vec![(token.start, token.end)] });
        }
        if prev.token == Token::Dot {
            return Some(JackSymbolAtCursor { kind: JackSymbolKind::Property, name: name.clone(), occurrences: vec![(token.start, token.end)] });
        }
    }
    let bound = jack_bound_variable_names(text);
    if !is_jack_variable_use_token(&tokens, index, &bound) {
        return None;
    }
    Some(JackSymbolAtCursor { kind: JackSymbolKind::Variable, name: name.clone(), occurrences: jack_variable_occurrences(text, name) })
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
fn editor_hover_context(document: &WriterProjection, runtime: &WriterPlayRuntime) -> (Option<String>, Option<(usize, usize)>, Vec<(usize, usize)>) {
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
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
semio_framework_program::app_labels! {
    /// 🗣️ Complete UI label set for the writer app; one field per label makes every locale combination compile-checked.
    struct WriterPlayLabels {
        document: &'static str = en: "Document", de: "Dokument";
        empty_query: &'static str = en: "(empty query)", de: "(leere Abfrage)";
        language: &'static str = en: "Language", de: "Sprache";
        jack_description: &'static str = en: "jack — Cypher-inspired trinity query language", de: "jack — von Cypher inspirierte Trinity-Abfragesprache";
        camera: &'static str = en: "Camera", de: "Kamera";
        diagnostics: &'static str = en: "Diagnostics", de: "Diagnosen";
        format: &'static str = en: "Format", de: "Formatieren";
        lint: &'static str = en: "Lint", de: "Prüfen";
        line_numbers: &'static str = en: "Line numbers", de: "Zeilennummern";
        font_size: &'static str = en: "Font size", de: "Schriftgröße";
        line_height: &'static str = en: "Line height", de: "Zeilenhöhe";
        tab_size: &'static str = en: "Tab size", de: "Tabulatorgröße";
        engagement_placeholder: &'static str = en: "Format, lint, line numbers", de: "Format, prüfen, Zeilennummern";
        editor_mode_status: &'static str = en: "Text editor", de: "Texteditor";
        window_main: &'static str = en: "Jack", de: "Jack";
        mode_edit: &'static str = en: "Edit", de: "Bearbeiten";
        panel_tab_content: &'static str = en: "Content", de: "Inhalt";
        panel_tab_outline: &'static str = en: "Outline", de: "Gliederung";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_writer_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the
/// command palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn writer_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("formatDocument", "Format Document", "Dokument formatieren"),
        ("lintDocument", "Lint Document", "Dokument prüfen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("textEdit", "Edit Text", "Text bearbeiten"),
        ("setText", "Set Text", "Text festlegen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("commitRename", "Commit Rename", "Umbenennung übernehmen"),
        ("engagementSubmit", "Engagement Submit", "Eingabe bestätigen"),
        ("setDocument", "Set Document", "Dokument festlegen"),
        ("setDocumentJson", "Set Document JSON", "Dokument-JSON festlegen"),
        ("setFixtureJson", "Set Fixture JSON", "Fixture-JSON festlegen"),
        ("requestCompletions", "Request Completions", "Vervollständigungen anfordern"),
        ("textSelect", "Text Select", "Text auswählen"),
        ("setEditorSelection", "Set Editor Selection", "Editor-Auswahl festlegen"),
        ("selectAstNode", "Select Ast Node", "AST-Knoten auswählen"),
        ("setAstSelection", "Set Ast Selection", "AST-Auswahl festlegen"),
        ("setAstHover", "Set Ast Hover", "Überfahren (AST) festlegen"),
        ("textHover", "Text Hover", "Text-Hover"),
        ("toggleLineNumbers", "Toggle Line Numbers", "Zeilennummern umschalten"),
        ("setEditorSetting", "Set Editor Setting", "Editor-Einstellung festlegen"),
        ("engagementInput", "Engagement Input", "Eingabe"),
    ];
    localized_label_map(is_de, ENTRIES)
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_writer_app`.
/// Writer declares no utilities today; kept for parity with the other apps' `app_labels()` wiring.
fn writer_utility_labels(_is_de: bool) -> HashMap<String, String> {
    HashMap::new()
}
//#endregion 🔖CommandLabels

//#region 🔖Panels
fn play_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: controller_id.into(),
        action: action.into(),
        args,
    }
}

fn render_document_panel(document: &WriterProjection, runtime: &WriterPlayRuntime, labels: &WriterPlayLabels) -> UiNode {
    if document.language_id != "jack" {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "writer-document".into(),
            label: Some(labels.document.into()),
            default_open: Some(true),
            children: vec![ui_text(document.id.clone()), ui_text(document.language_id.clone())],
            presence: UiPresence::default(),
}]);
    }
    let root = parse_jack_ast(&document.text);
    let items = if root.kind == "error" {
        vec![UiTreeItemNode {
            description: Some(root.kind.clone()),
            icon_id: jack_ast_tree_icon(&root.kind).and_then(IconName::from_str),
            ..tree_item(root.id.as_str(), root.label.as_str())
        }]
    } else {
        vec![jack_ast_to_tree_item(&root)]
    };
    let (highlighted_ast_id, _, _) = editor_hover_context(document, runtime);
    PanelTreeBuilder::new("writer-play-document")
        .section_or_placeholder("writer-play-document.ast", Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL.into()), true, items, labels.empty_query)
        .selected(runtime.selected_ast_ids.clone())
        .highlighted(highlighted_ast_id.map(|id| vec![id]).unwrap_or_default())
        .selection_change(play_action(WRITER_PLAY_CONTROLLER_ID, "setAstSelection", None))
        .build()
}

fn render_catalogue_panel(labels: &WriterPlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "writer-catalogue".into(),
        label: Some(labels.language.into()),
        default_open: Some(true),
        children: vec![ui_text(labels.jack_description)],
        presence: UiPresence::default(),
}])
}

fn render_inspection_panel(document: &WriterProjection, labels: &WriterPlayLabels) -> UiNode {
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
            presence: UiPresence::default(),
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
            presence: UiPresence::default(),
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
                presence: UiPresence::default(),
});
        }
    }
    ui_declarative_sections_to_tree(&sections)
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_scene(document: &WriterProjection, runtime: &WriterPlayRuntime) -> UiNode {
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
//#endregion 🔖Render

//#region 🔖Engagement
/// 💬 Natural-language engagement parsing (premigration `applyEngagement`). Accepts both the
/// spaced form (wgpu REPL) and the React shell's PascalCased, separator-stripped drafts (e.g.
/// `"Font16"`, `"LineNumbers"` — see `strip_engagement_prefix`). Mutates ephemeral `runtime`
/// state in place; returns `Some(new_text)` only for the `format` branch when the source changed,
/// so the caller can emit a `SetText` operation — every other branch returns `None` (view-only).
fn apply_engagement(runtime: &mut WriterPlayRuntime, current_text: &str, language_id: &str, value: &str) -> Option<String> {
    let trimmed = value.trim();
    runtime.engagement_input.clear();
    runtime.revision += 1;
    if trimmed.is_empty() {
        return None;
    }
    if engagement_token_matches(trimmed, "format") {
        runtime.format_signal += 1;
        let formatted = format_writer_text(current_text, language_id);
        return (formatted != current_text).then_some(formatted);
    }
    if engagement_token_matches(trimmed, "lint") {
        runtime.lint_signal += 1;
        return None;
    }
    if engagement_token_matches(trimmed, "line numbers") || engagement_token_matches(trimmed, "numbers") || engagement_token_matches(trimmed, "gutter") {
        runtime.editor_settings.show_line_numbers = !runtime.editor_settings.show_line_numbers;
        return None;
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "font size").or_else(|| strip_engagement_prefix(trimmed, "font")) {
        if let Ok(px) = rest.parse::<u32>() {
            runtime.editor_settings.font_px = px;
        }
        return None;
    }
    if let Some(rest) = strip_engagement_prefix(trimmed, "tab size").or_else(|| strip_engagement_prefix(trimmed, "tab")) {
        if let Ok(size) = rest.parse::<u32>() {
            runtime.editor_settings.tab_size = size.max(1);
        }
    }
    None
}
//#endregion 🔖Engagement

//#region 🔖WriterPlayApp
#[derive(Default)]
struct WriterPlayApp {
    /// 🎛️ Ephemeral view state (selection, hover, editor settings, signals, engagement draft) that
    /// lives on the app struct — never in the document projection, so it emits no history entries.
    runtime: WriterPlayRuntime,
}

impl DocumentApp for WriterPlayApp {
    type Projection = WriterProjection;
    type Operation = WriterOperation;

    fn app_id(&self) -> &str {
        WRITER_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        WRITER_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> WriterProjection {
        empty_writer_projection()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, WriterProjection>,
        _view_state: &ViewState,
    ) -> ActionEmit<WriterOperation> {
        // undo/redo/checkpoint/alternative never reach here — `VcsDocumentApp` intercepts them.
        let document = doc.projection;
        let str_arg = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str);
        let usize_arg = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_u64).map(|value| value as usize);
        match action {
            "textEdit" => {
                if let Some(text) = str_arg("text") {
                    // ⌨️ Keystroke-granular edits coalesce under a stable key so a typing burst amends into
                    // a few undo steps, not one-per-keystroke. Any interrupting action (format, example
                    // load, engagement submit) applies without this key and breaks the coalescing run.
                    return ActionEmit::amend(vec![WriterOperation::SetText { text: text.into() }], "writer-text-edit");
                }
                ActionEmit::default()
            }
            "setText" => {
                if let Some(text) = str_arg("text") {
                    // 🪙 A discrete document replacement (unlike `textEdit`'s keystroke bursts) — each call
                    // is its own undo step, so it must NOT share `textEdit`'s coalescing key.
                    return ActionEmit::operations(vec![WriterOperation::SetText { text: text.into() }]);
                }
                ActionEmit::default()
            }
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<WriterProjection>(next.clone()) {
                        return ActionEmit::operations(vec![WriterOperation::SetDocument { document: parsed }]);
                    }
                }
                ActionEmit::default()
            }
            "setDocumentJson" | "setFixtureJson" => {
                if let Some(json_text) = str_arg("json") {
                    if let Ok(parsed) = serde_json::from_str::<WriterProjection>(json_text) {
                        return ActionEmit::operations(vec![WriterOperation::SetDocument { document: parsed }]);
                    }
                }
                ActionEmit::default()
            }
            "setActiveExample" => {
                let example_id = str_arg("exampleId").unwrap_or("");
                let document = match example_id {
                    "jack" => jack_example_document(),
                    "dag.jack" => dag_jack_example_document(),
                    _ => empty_writer_projection(),
                };
                ActionEmit::operations(vec![WriterOperation::SetDocument { document }])
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value::<WriterCamera>(camera.clone()) {
                        // 🎥 Camera is a doc operation by policy; a pan/zoom drag coalesces into one undo step.
                        return ActionEmit::amend(vec![WriterOperation::SetCamera { camera: parsed }], "writer-camera");
                    }
                }
                ActionEmit::default()
            }
            "formatDocument" => {
                self.runtime.format_signal += 1;
                let formatted = format_writer_text(&document.text, &document.language_id);
                if formatted != document.text {
                    return ActionEmit::operations(vec![WriterOperation::SetText { text: formatted }]);
                }
                ActionEmit::default()
            }
            "commitRename" => {
                let Some(new_text) = str_arg("text") else {
                    return ActionEmit::default();
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
                    let text = apply_jack_rename(&document.text, &occurrences, new_text);
                    return ActionEmit::operations(vec![WriterOperation::SetText { text }]);
                }
                if let (Some(start), Some(end)) = (usize_arg("start"), usize_arg("end")) {
                    if start <= end && end <= document.text.len() {
                        let mut text = document.text.clone();
                        text.replace_range(start..end, new_text);
                        return ActionEmit::operations(vec![WriterOperation::SetText { text }]);
                    }
                }
                ActionEmit::default()
            }
            "requestCompletions" => {
                self.runtime.revision += 1;
                ActionEmit::default()
            }
            "lintDocument" => {
                self.runtime.lint_signal += 1;
                self.runtime.revision += 1;
                ActionEmit::default()
            }
            "textSelect" | "setEditorSelection" => {
                let start = usize_arg("start").unwrap_or(0);
                let end = usize_arg("end").unwrap_or(start);
                self.runtime.editor_selection = Some(WriterEditorSelection { start, end });
                if document.language_id == "jack" {
                    let root = parse_jack_ast(&document.text);
                    self.runtime.selected_ast_ids = jack_ast_node_for_selection(&root, start.min(end), start.max(end))
                        .map(|node| vec![node.id.clone()])
                        .unwrap_or_default();
                } else {
                    self.runtime.selected_ast_ids.clear();
                }
                self.runtime.revision += 1;
                ActionEmit::default()
            }
            "selectAstNode" => {
                let id = str_arg("id").unwrap_or("");
                let start = usize_arg("start").unwrap_or(0);
                let end = usize_arg("end").unwrap_or(0);
                self.runtime.selected_ast_ids = if id.is_empty() { Vec::new() } else { vec![id.into()] };
                self.runtime.editor_selection = Some(WriterEditorSelection { start, end });
                self.runtime.revision += 1;
                ActionEmit::default()
            }
            "setAstSelection" => {
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| value.as_array())
                    .map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                self.runtime.selected_ast_ids = ids.clone();
                if let Some(id) = ids.first() {
                    if document.language_id == "jack" {
                        let root = parse_jack_ast(&document.text);
                        self.runtime.editor_selection = jack_ast_node_by_id(&root, id).map(|node| WriterEditorSelection { start: node.start, end: node.end });
                    }
                }
                self.runtime.revision += 1;
                ActionEmit::default()
            }
            "setAstHover" => {
                let id = str_arg("id").map(str::to_string);
                if id != self.runtime.tree_hovered_ast_id {
                    self.runtime.tree_hovered_ast_id = id;
                    self.runtime.revision += 1;
                }
                ActionEmit::default()
            }
            "textHover" => {
                let start = usize_arg("start");
                let end = usize_arg("end");
                let offset = match (start, end) {
                    (Some(s), Some(e)) => Some(s + e.saturating_sub(s) / 2),
                    _ => None,
                };
                if offset != self.runtime.editor_hover_offset {
                    self.runtime.editor_hover_offset = offset;
                    self.runtime.revision += 1;
                }
                ActionEmit::default()
            }
            "toggleLineNumbers" => {
                self.runtime.editor_settings.show_line_numbers = !self.runtime.editor_settings.show_line_numbers;
                self.runtime.revision += 1;
                ActionEmit::default()
            }
            "setEditorSetting" => {
                let field = str_arg("field").unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                match field {
                    "fontPx" => {
                        if let Some(px) = value.and_then(Value::as_u64) {
                            self.runtime.editor_settings.font_px = px as u32;
                        }
                    }
                    "lineHeight" => {
                        if let Some(px) = value.and_then(Value::as_u64) {
                            self.runtime.editor_settings.line_height = px as u32;
                        }
                    }
                    "tabSize" => {
                        if let Some(px) = value.and_then(Value::as_u64) {
                            self.runtime.editor_settings.tab_size = px.max(1) as u32;
                        }
                    }
                    _ => return ActionEmit::default(),
                }
                self.runtime.revision += 1;
                ActionEmit::default()
            }
            "engagementInput" => {
                let value = str_arg("value").unwrap_or("").to_string();
                if value != self.runtime.engagement_input {
                    self.runtime.engagement_input = value;
                    self.runtime.revision += 1;
                }
                ActionEmit::default()
            }
            "engagementSubmit" => {
                let value = str_arg("value").map(str::to_string).unwrap_or_else(|| self.runtime.engagement_input.clone());
                match apply_engagement(&mut self.runtime, &document.text, &document.language_id, &value) {
                    Some(text) => ActionEmit::operations(vec![WriterOperation::SetText { text }]),
                    None => ActionEmit::default(),
                }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, WriterProjection>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = resolve_labels::<WriterPlayLabels>(view_state);
        match body_key {
            WRITER_PLAY_BODY_MAIN => render_main_scene(document, &self.runtime),
            WRITER_PLAY_BODY_DOCUMENT => render_document_panel(document, &self.runtime, labels),
            WRITER_PLAY_BODY_CATALOGUE => render_catalogue_panel(labels),
            WRITER_PLAY_BODY_INSPECTION => render_inspection_panel(document, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, _doc: &DocumentView<'_, WriterProjection>, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let settings = &self.runtime.editor_settings;
        let labels = resolve_labels::<WriterPlayLabels>(view_state);
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
                value: Some(self.runtime.engagement_input.clone()),
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

    fn window_measures(&self, _doc: &DocumentView<'_, WriterProjection>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let settings = &self.runtime.editor_settings;
        let labels = resolve_labels::<WriterPlayLabels>(view_state);
        let measures = vec![
            WindowMeasure::Slider {
                id: "writer-font-size-measure".into(),
                label: Some(labels.font_size.into()),
                value: settings.font_px as f64,
                min: 10.0,
                max: 24.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "setEditorSetting", Some(json!({ "field": "fontPx" }))),
                },
            WindowMeasure::Slider {
                id: "writer-line-height-measure".into(),
                label: Some(labels.line_height.into()),
                value: settings.line_height as f64,
                min: 16.0,
                max: 40.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: play_action(WRITER_PLAY_CONTROLLER_ID, "setEditorSetting", Some(json!({ "field": "lineHeight" }))),
                },
            WindowMeasure::Slider {
                id: "writer-tab-size-measure".into(),
                label: Some(labels.tab_size.into()),
                value: settings.tab_size as f64,
                min: 1.0,
                max: 8.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
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

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<WriterPlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(WRITER_PLAY_WINDOW_KIND, labels.window_main)
            .panel_tab_label(WRITER_PANEL_TAB_DOCUMENT_CONTENT_ID, labels.panel_tab_content)
            .panel_tab_label(WRITER_PANEL_TAB_DOCUMENT_OUTLINE_ID, labels.panel_tab_outline)
            .mode_label("edit", labels.mode_edit)
            .action_labels(writer_action_labels(is_de))
            .utility_labels(writer_utility_labels(is_de))
    }
}
//#endregion 🔖WriterPlayApp

//#region 🔖Manifest
/// 🙈 An internal document operation kept out of the command palette — editor events (text edits,
/// camera, rename, engagement submit) and dev-only whole-document setters dispatched from chrome.
fn writer_hidden_operation(id: &str, label: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, ActionKind::Operation) }
}

/// 🙈 An internal View action kept out of the palette — ephemeral editor/selection/hover/setting events
/// that mutate only runtime scratch and emit no document operations.
fn writer_hidden_view(id: &str, label: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, ActionKind::View) }
}

fn create_writer_app() -> App {
    App::from_builder(
        App::builder(WRITER_PLAY_APP_ID, "Writer").document(["semio", "writer"])
            .artifact_kind(ArtifactKindSpec {
                id: "text.document".into(),
                name: "Text Document".into(),
                source_format: "writer.document".into(),
                component_kind: "writer".into(),
                dimension: "text".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
                schema: "writer.document".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("writer")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(WRITER_PLAY_WINDOW_KIND, "Jack", WRITER_PLAY_BODY_MAIN, SurfaceKind::TextEditor, "document-jack")
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
            // 🔧 Panel-visible P0 effects: format rewrites the buffer (Operation), lint re-runs
            // diagnostics into runtime (View — an effect, not a document operation).
            .operation("formatDocument", "Format Document")
            .view_action("lintDocument", "Lint Document")
            // 🔧 P1 example switch (whole-document load) with a staged example choice.
            .operation("setActiveExample", "Set Active Example")
            // 🙈 Internal document operations — text edits (coalesced), aliases, camera, rename, engagement,
            // and dev-only whole-document JSON setters.
            .action_with(writer_hidden_operation("textEdit", "Edit Text"))
            .action_with(writer_hidden_operation("setText", "Set Text"))
            .action_with(writer_hidden_operation("setCamera", "Set Camera"))
            .action_with(writer_hidden_operation("commitRename", "Commit Rename"))
            .action_with(writer_hidden_operation("engagementSubmit", "Engagement Submit"))
            .action_with(writer_hidden_operation("setDocument", "Set Document"))
            .action_with(writer_hidden_operation("setDocumentJson", "Set Document JSON"))
            .action_with(writer_hidden_operation("setFixtureJson", "Set Fixture JSON"))
            // 🙈 Internal View measures — selection, hover, AST navigation, completions, editor settings.
            .action_with(writer_hidden_view("requestCompletions", "Request Completions"))
            .action_with(writer_hidden_view("textSelect", "Text Select"))
            .action_with(writer_hidden_view("setEditorSelection", "Set Editor Selection"))
            .action_with(writer_hidden_view("selectAstNode", "Select Ast Node"))
            .action_with(writer_hidden_view("setAstSelection", "Set Ast Selection"))
            .action_with(writer_hidden_view("setAstHover", "Set Ast Hover"))
            .action_with(writer_hidden_view("textHover", "Text Hover"))
            .action_with(writer_hidden_view("toggleLineNumbers", "Toggle Line Numbers"))
            .action_with(writer_hidden_view("setEditorSetting", "Set Editor Setting"))
            .action_with(writer_hidden_view("engagementInput", "Engagement Input"))
            // 📝 Staged argument forms: example choice + the dev JSON setters.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new("jack", "Jack"),
                    ActionArgOption::new("dag.jack", "Dag Jack"),
                ]).default_value("jack"),
            ])
            .action_args("setDocumentJson", vec![ActionArgDef::text("json", "Document JSON")])
            .action_args("setFixtureJson", vec![ActionArgDef::text("json", "Fixture JSON")])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("jack", "Jack", jack_example_json())
    .example("dag.jack", "Dag Jack", dag_jack_example_json())
    .workflow("writer", "Writer", "text.document")
}

/// 🗂️ Registers `WriterProjection`'s pack↔dsl codec so `framework/sync`'s `FolderEndpoint::Pack`
/// (and any other schema-string-keyed caller) can print/parse it without depending on this crate's
/// concrete `Projection`/`Operation` types.
fn register_writer_exports() {
    semio_framework_program::program_runtime::register_document_codec_for_app::<WriterPlayApp>(WRITER_DOCUMENT_SCHEMA);
}

semio_framework_program::semio_program! {
    id: "writer", label: "Writer", version: "0.1.0",
    setup: register_writer_exports,
    apps: [ create_writer_app => WriterPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_program::{
        testkit::{meta, new_app, new_app_with_registry},
        ProgramApp, VcsDocumentApp,
    };

    /// ✍️ Loads the canonical jack fixture into the store, returning the app ready to exercise.
    fn app_with_jack() -> VcsDocumentApp<WriterPlayApp> {
        let mut app = new_app::<WriterPlayApp>();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "jack" })), &ViewState::default(), &meta("local")).expect("load jack");
        app
    }

    #[test]
    fn text_edit_burst_coalesces_into_one_undo_step() {
        let mut app = new_app::<WriterPlayApp>();
        for text in ["h", "he", "hel", "hell", "hello"] {
            app.handle_action("textEdit", Some(&json!({ "text": text })), &ViewState::default(), &meta("local")).expect("type");
        }
        assert_eq!(app.projection().expect("projection").text, "hello");
        // The whole typing burst shares one coalesce key, so a single undo restores the pre-burst buffer
        // rather than backing out one keystroke at a time.
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").text, "", "coalesced typing collapses to one undo step");
    }

    #[test]
    fn lint_is_a_view_action_and_example_default_materializes() {
        let mut app = new_app_with_registry::<WriterPlayApp>(create_writer_app);
        // lintDocument is a declared View action: registry kind discipline requires it emit no operations.
        let result = app.handle_action("lintDocument", None, &ViewState::default(), &meta("local")).expect("lint");
        assert!(result.operations.is_empty(), "lint re-runs diagnostics into runtime, never the document");
        // setActiveExample fired with no args materializes the declared default example ("jack").
        app.handle_action("setActiveExample", None, &ViewState::default(), &meta("local")).expect("example");
        assert!(!app.projection().expect("projection").text.is_empty(), "jack default materialized from the registry");
    }

    #[test]
    fn renders_text_editor_scene() {
        let mut app = new_app::<WriterPlayApp>();
        let node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn renders_document_tree_for_jack() {
        let mut app = new_app::<WriterPlayApp>();
        let node = app.render(WRITER_PLAY_BODY_DOCUMENT, Some(&jack_example_json()), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Query"));
    }

    #[test]
    fn renders_catalogue_panel() {
        let mut app = new_app::<WriterPlayApp>();
        let node = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("jack"));
    }

    #[test]
    fn format_document_reformats_jack_query() {
        let mut app = app_with_jack();
        app.handle_action("setText", Some(&json!({ "text": "MATCH (a:Piece)   WHERE a.name='core' RETURN a.name" })), &ViewState::default(), &meta("local")).expect("set text");
        let result = app.handle_action("formatDocument", None, &ViewState::default(), &meta("local")).expect("format");
        assert_eq!(result.operations.len(), 1);
        assert!(app.projection().expect("projection").text.contains('\n'));
    }

    #[test]
    fn format_document_without_change_emits_no_operation() {
        // A no-operation format (already-formatted or non-jack empty doc) bumps the format signal but must
        // not record a history entry.
        let mut app = new_app::<WriterPlayApp>();
        let result = app.handle_action("formatDocument", None, &ViewState::default(), &meta("local")).expect("format");
        assert!(result.operations.is_empty());
    }

    #[test]
    fn jack_completions_use_example_fixture() {
        let json = jack_completions_json("RETURN a.", 9).unwrap_or_default();
        assert!(!json.is_empty());
    }

    #[test]
    fn set_text_action_updates_projection() {
        let mut app = new_app::<WriterPlayApp>();
        let result = app.handle_action("setText", Some(&json!({ "text": "MATCH (a) RETURN a" })), &ViewState::default(), &meta("local")).expect("set text");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").text, "MATCH (a) RETURN a");
    }

    #[test]
    fn set_text_undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app::<WriterPlayApp>();
        app.handle_action("setText", Some(&json!({ "text": "first" })), &ViewState::default(), &meta("local")).expect("first");
        app.handle_action("setText", Some(&json!({ "text": "second" })), &ViewState::default(), &meta("local")).expect("second");
        assert_eq!(app.projection().expect("projection").text, "second");
        let undo = app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert!(undo.operations.is_empty());
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.projection().expect("projection").text, "first");
        app.handle_action("redo", None, &ViewState::default(), &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").text, "second");
    }

    #[test]
    fn set_camera_action_updates_projection() {
        let mut app = new_app::<WriterPlayApp>();
        let result = app.handle_action("setCamera", Some(&json!({ "camera": { "x": 3.0, "y": 4.0, "zoom": 2.0 } })), &ViewState::default(), &meta("local")).expect("set camera");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.camera.x, 3.0);
        assert_eq!(projection.camera.zoom, 2.0);
    }

    #[test]
    fn view_action_emits_no_operations() {
        let mut app = new_app::<WriterPlayApp>();
        let result = app.handle_action("toggleLineNumbers", None, &ViewState::default(), &meta("local")).expect("toggle");
        assert!(result.operations.is_empty());
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
        let mut app = app_with_jack();
        let occurrences = jack_variable_occurrences(CANONICAL_QUERY, "a");
        assert_eq!(occurrences.len(), 3);
        let occurrences_json: Vec<Value> = occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
        let result = app.handle_action(
            "commitRename",
            Some(&json!({ "occurrences": occurrences_json, "text": "piece" })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("commit rename");
        assert_eq!(result.operations.len(), 1);
        let text = app.projection().expect("projection").text;
        assert_eq!(text.matches("piece").count(), 3);
        assert_eq!(text.matches("a:Piece").count(), 0);
    }

    #[test]
    fn engagement_submit_parses_font_size() {
        let mut app = new_app::<WriterPlayApp>();
        let result = app.handle_action("engagementSubmit", Some(&json!({ "value": "font 16" })), &ViewState::default(), &meta("local")).expect("submit");
        // Font size is ephemeral view state — no history entry.
        assert!(result.operations.is_empty());
        let measures = app.window_measures(&ViewState::default());
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
    }

    #[test]
    fn engagement_submit_parses_normalized_shell_drafts() {
        // The React shell PascalCases and strips separators from every draft before submitting it
        // (`normalizeEngagementActionText`), so "font 16" arrives as "Font16", "tab 4" as "Tab4",
        // and "line numbers" as "LineNumbers".
        let mut app = new_app::<WriterPlayApp>();
        let before_toggle = app
            .window_engagements(&ViewState::default())
            .get(WRITER_PLAY_WINDOW_KIND)
            .and_then(|engagement| engagement.options.as_ref())
            .and_then(|options| options.first())
            .and_then(|option| option.pressed)
            .expect("line-numbers pressed state");

        app.handle_action("engagementSubmit", Some(&json!({ "value": "Font16" })), &ViewState::default(), &meta("local")).expect("font");
        app.handle_action("engagementSubmit", Some(&json!({ "value": "Tab4" })), &ViewState::default(), &meta("local")).expect("tab");
        app.handle_action("engagementSubmit", Some(&json!({ "value": "LineNumbers" })), &ViewState::default(), &meta("local")).expect("line numbers");

        let measures = app.window_measures(&ViewState::default());
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-tab-size-measure" && *value == 4.0)));

        let after_toggle = app
            .window_engagements(&ViewState::default())
            .get(WRITER_PLAY_WINDOW_KIND)
            .and_then(|engagement| engagement.options.as_ref())
            .and_then(|options| options.first())
            .and_then(|option| option.pressed)
            .expect("line-numbers pressed state");
        assert_eq!(after_toggle, !before_toggle);
    }

    #[test]
    fn window_measures_expose_font_line_height_tab_and_toggle() {
        let mut app = new_app::<WriterPlayApp>();
        let measures = app.window_measures(&ViewState::default());
        let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
        assert_eq!(main.len(), 4);
        assert!(main.iter().any(|m| matches!(m, WindowMeasure::Toggle { id, .. } if id == "writer-line-numbers-measure")));
    }

    #[test]
    fn window_engagements_expose_format_lint_placeholder() {
        let mut app = new_app::<WriterPlayApp>();
        let engagements = app.window_engagements(&ViewState::default());
        let main = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("main engagement");
        let placeholder = main.input.as_ref().and_then(|i| i.placeholder.as_ref()).expect("placeholder");
        assert!(placeholder.contains("Format"));
        assert_eq!(main.possible_engagements.as_ref().map(|v| v.len()), Some(3));
    }

    // 🧰 `VcsDocumentApp::tools()` (a per-app custom utility bar) no longer exists — utility bars
    // are now derived by the renderer from the utility registry (`writer_utility_labels` above;
    // writer declares no utilities). Format/lint were never single-sourced from that removed
    // utility bar though: they're `WindowEngagementPossible` entries in `window_engagements()`,
    // which is still the one surface for them — assert on that surface instead.
    #[test]
    fn window_engagements_include_format_and_lint_possible_engagements() {
        let mut app = new_app::<WriterPlayApp>();
        let engagements = app.window_engagements(&ViewState::default());
        let engagement = engagements.get(WRITER_PLAY_WINDOW_KIND).expect("writer window engagement");
        let ids: Vec<&str> = engagement.possible_engagements.as_ref().expect("possible engagements").iter().map(|possible| possible.id.as_str()).collect();
        assert!(ids.contains(&"writer-format"));
        assert!(ids.contains(&"writer-lint"));
    }

    #[test]
    fn scene_emits_placeholders_selectable_spans_and_newline_gates_for_jack() {
        let mut app = new_app::<WriterPlayApp>();
        let node = app.render(WRITER_PLAY_BODY_MAIN, Some(&jack_example_json()), &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("placeholdersJson"));
        assert!(json.contains("selectableSpansJson"));
        assert!(json.contains("newlineGatesJson"));
    }

    #[test]
    fn set_ast_hover_updates_tree_highlight_and_scene_hover() {
        let mut app = app_with_jack();
        let root = parse_jack_ast(&app.projection().expect("projection").text);
        let result = app.handle_action("setAstHover", Some(&json!({ "id": root.id })), &ViewState::default(), &meta("local")).expect("hover");
        assert!(result.operations.is_empty());
        let tree_node = app.render(WRITER_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render tree");
        let tree_json = serde_json::to_string(&tree_node).unwrap();
        assert!(tree_json.contains(&root.id));
        let scene_node = app.render(WRITER_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render scene");
        let scene_value = serde_json::to_value(&scene_node).unwrap();
        let hover_json = scene_value["textEditor"]["hoverJson"].as_str().expect("hoverJson string");
        let hover_range: Value = serde_json::from_str(hover_json).unwrap();
        assert_eq!(hover_range["start"].as_u64(), Some(root.start as u64));
        assert_eq!(hover_range["end"].as_u64(), Some(root.end as u64));
    }

    #[test]
    fn manifest_includes_dag_jack_example() {
        let bundle = __semio_program_bundle();
        let manifest = &bundle.manifest;
        assert!(manifest.apps.iter().any(|a| a.id == WRITER_PLAY_APP_ID));
        assert!(manifest.examples.iter().any(|e| e.id == "dag.jack" && e.app_id == WRITER_PLAY_APP_ID));
    }

    #[test]
    fn set_active_example_loads_jack_fixture() {
        let mut app = new_app::<WriterPlayApp>();
        let result = app.handle_action("setActiveExample", Some(&json!({ "exampleId": "jack" })), &ViewState::default(), &meta("local")).expect("load");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.id, "jack");
        assert!(projection.text.contains("MATCH"));
    }

    #[test]
    fn set_active_example_loads_dag_jack_fixture() {
        let mut app = new_app::<WriterPlayApp>();
        let result = app.handle_action("setActiveExample", Some(&json!({ "exampleId": "dag.jack" })), &ViewState::default(), &meta("local")).expect("load");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").id, "dag-jack");
    }

    #[test]
    fn set_active_example_falls_back_to_empty_document() {
        let mut app = app_with_jack();
        let result = app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &ViewState::default(), &meta("local")).expect("load");
        assert_eq!(result.operations.len(), 1);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.id, "empty");
        assert_eq!(projection.text, "");
    }

    #[test]
    fn writer_labels_resolve_native_by_default() {
        let mut app = new_app::<WriterPlayApp>();
        let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("\"Document\""));
        assert!(inspection_json.contains("\"Camera\""));
        let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("\"Language\""));
        assert!(catalogue_json.contains("Cypher-inspired"));
        let engagements = app.window_engagements(&ViewState::default());
        let engagements_json = serde_json::to_string(&engagements).unwrap();
        assert!(engagements_json.contains("\"Format\""));
        assert!(engagements_json.contains("\"Lint\""));
        let measures = app.window_measures(&ViewState::default());
        let measures_json = serde_json::to_string(&measures).unwrap();
        assert!(measures_json.contains("Font size"));
        assert!(measures_json.contains("Line numbers"));
        assert!(!measures_json.contains("Schriftgröße"));
    }

    #[test]
    fn writer_labels_resolve_german_locale() {
        let mut app = new_app::<WriterPlayApp>();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, None, &view_state).expect("render");
        let inspection_json = serde_json::to_string(&inspection).unwrap();
        assert!(inspection_json.contains("Dokument"));
        assert!(inspection_json.contains("Kamera"));
        assert!(!inspection_json.contains("\"Camera\""));
        let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &view_state).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Sprache"));
        let measures = app.window_measures(&view_state);
        let measures_json = serde_json::to_string(&measures).unwrap();
        assert!(measures_json.contains("Schriftgröße"));
        assert!(measures_json.contains("Zeilennummern"));
        let engagements = app.window_engagements(&view_state);
        let engagements_json = serde_json::to_string(&engagements).unwrap();
        assert!(engagements_json.contains("Texteditor"));
        assert!(engagements_json.contains("Formatieren"));
        assert!(engagements_json.contains("Prüfen"));
    }
}
//#endregion 🧪Tests
