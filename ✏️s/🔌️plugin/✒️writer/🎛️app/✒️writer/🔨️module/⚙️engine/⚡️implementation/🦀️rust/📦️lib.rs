//! ⚙️ Writer app — headless compute (constitutional: engine).

mod grammar {
    //#region 🔖️Grammar
    //! 🎨️ Lightweight grammar tokenization for writer program scenes.

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

pub use grammar::{tokenize_language, GrammarToken};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use writer::{WriterProjection, WRITER_DOCUMENT_SCHEMA};

//#region 🔖Examples
/// 📄 The `jack` example, parsed once from {@link writer_dsl::JACK_EXAMPLE_TEXT} — the source of truth
/// for every call site below (`setActiveExample`, `.example("jack", ...)`, tests); never re-embed the
/// raw text.
pub fn jack_example_document() -> WriterProjection {
    <WriterProjection as store::DocumentDsl>::parse_dsl(writer_dsl::JACK_EXAMPLE_TEXT).unwrap_or_else(|_| empty_writer_projection())
}

/// 📄 JSON re-serialization of {@link jack_example_document}, for the framework-generic call sites
/// (`.example(...)`, `render(...)`) that still take a document as a JSON string.
pub fn jack_example_json() -> String {
    serde_json::to_string(&jack_example_document()).expect("serialize jack example document")
}

/// 📄 The `dag.jack` example, parsed once from {@link writer_dsl::DAG_JACK_EXAMPLE_TEXT} — see
/// {@link jack_example_document}.
pub fn dag_jack_example_document() -> WriterProjection {
    <WriterProjection as store::DocumentDsl>::parse_dsl(writer_dsl::DAG_JACK_EXAMPLE_TEXT).unwrap_or_else(|_| empty_writer_projection())
}

/// 📄 JSON re-serialization of {@link dag_jack_example_document} — see {@link jack_example_json}.
pub fn dag_jack_example_json() -> String {
    serde_json::to_string(&dag_jack_example_document()).expect("serialize dag.jack example document")
}
//#endregion 🔖Examples

//#region 🔖DocumentHelpers
pub fn empty_writer_projection() -> WriterProjection {
    WriterProjection { schema: WRITER_DOCUMENT_SCHEMA.into(), id: "empty".into(), language_id: "plaintext".into(), uri: "writer://empty".into(), text: String::new() }
}

//#region 🔖JackAst
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JackAstNode {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub start: usize,
    pub end: usize,
    #[serde(default)]
    pub children: Vec<JackAstNode>,
}

pub fn jack_ast_tree_icon(kind: &str) -> Option<&'static str> {
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

/// 🌉️ Adapts trinity_jack's shared [`trinity_jack::SpannedNode`] tree into writer's own [`JackAstNode`]
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

/// 🌳️ Parse jack source into a span-tracked AST for hierarchy panels, via the shared `trinity_jack` parser.
pub fn parse_jack_ast(text: &str) -> JackAstNode {
    jack_ast_from_spanned(&trinity_jack::parse_spanned(text))
}

/// 🎯️ Deepest AST node containing a byte offset.
pub fn find_deepest_jack_ast_node_at(root: &JackAstNode, offset: usize) -> Option<&JackAstNode> {
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

/// 🔎️ Find an AST node by stable id.
pub fn jack_ast_node_by_id<'a>(root: &'a JackAstNode, id: &str) -> Option<&'a JackAstNode> {
    if root.id == id {
        return Some(root);
    }
    root.children.iter().find_map(|child| jack_ast_node_by_id(child, id))
}

/// 🖱️ Smallest AST node that fully contains a selection range.
pub fn jack_ast_node_for_selection<'a>(root: &'a JackAstNode, start: usize, end: usize) -> Option<&'a JackAstNode> {
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
//#endregion 🔖️JackAst

//#region 🔖️JackEditor
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectableSpan {
    pub start: usize,
    pub end: usize,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_end: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_start: Option<usize>,
}

/// 🎯️ Builds atomic and composite jack spans for token-wise selection (premigration `selectableSpansForJack`).
pub fn selectable_spans_for_jack(text: &str, tokens: &[GrammarToken]) -> Vec<SelectableSpan> {
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
pub struct JackEditorPlaceholder {
    pub offset: usize,
    pub label: String,
}

fn jack_placeholder_visible(caret: usize, offset: usize) -> bool {
    let caret = caret as i64;
    let offset = offset as i64;
    caret >= offset - 32 && caret <= offset + 48
}

/// 🔤️ Fine-grained, never-fails jack tokens for editor heuristics — routed through `trinity_jack`'s shared
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

/// 👻️ Required jack token placeholders near the caret (premigration `jackEditorPlaceholders`).
pub fn jack_editor_placeholders(text: &str, caret: usize) -> Vec<JackEditorPlaceholder> {
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
pub fn jack_newline_allowed_at(text: &str, offset: usize) -> bool {
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
pub fn jack_newline_gate_offsets(text: &str) -> Vec<usize> {
    (0..=text.len()).filter(|&offset| text.is_char_boundary(offset) && jack_newline_allowed_at(text, offset)).collect()
}

/// 🔗️ Bound jack variable names from pattern bindings (premigration `jackBoundVariableNames`).
pub fn jack_bound_variable_names(text: &str) -> std::collections::HashSet<String> {
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

/// 🔁️ All bound-variable occurrences for a jack variable name (premigration `jackVariableOccurrences`).
pub fn jack_variable_occurrences(text: &str, var_name: &str) -> Vec<(usize, usize)> {
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
pub enum JackSymbolKind {
    Variable,
    Property,
    NodeKind,
    EdgeKind,
}

#[derive(Clone, Debug)]
pub struct JackSymbolAtCursor {
    pub kind: JackSymbolKind,
    pub name: String,
    pub occurrences: Vec<(usize, usize)>,
}

/// 🎯️ Resolve the jack symbol at a byte offset for semantic editor actions (premigration `jackSymbolAtOffset`).
pub fn jack_symbol_at_offset(text: &str, offset: usize) -> Option<JackSymbolAtCursor> {
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
pub fn apply_jack_rename(text: &str, occurrences: &[(usize, usize)], new_name: &str) -> String {
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

pub fn jack_completions_json(text: &str, cursor: usize) -> Option<String> {
    let graph = trinity_jack::example_graph();
    let items: Vec<Value> = trinity_jack::complete(&graph, text, cursor)
        .into_iter()
        .map(|item| json!({ "label": item.label, "detail": item.detail }))
        .collect();
    serde_json::to_string(&items).ok()
}

/// 🪞️ Canonical jack format when possible, else a whitespace-only normalization for other languages.
pub fn format_writer_text(text: &str, language_id: &str) -> String {
    if language_id == "jack" {
        if let Ok(formatted) = trinity_jack::format(text) {
            return formatted;
        }
    }
    let mut normalized: String = text.lines().map(|line| line.trim_end()).collect::<Vec<_>>().join("\n");
    if !text.is_empty() && !normalized.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}
//#endregion 🔖️JackEditor
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

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
    fn jack_completions_use_example_fixture() {
        let json = jack_completions_json("RETURN a.", 9).unwrap_or_default();
        assert!(!json.is_empty());
    }
}
//#endregion 🧪️Tests
