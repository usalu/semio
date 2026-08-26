//! 🧬️ Writer artifact schema — every field with its state class.

use crate::artifacts::writer::{document_child_handle_and_cache, WriterDocumentChild, WriterEditorSelection, WriterEditorSettings, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use trinity::lexer::{lex_spanned, SpannedToken, Token};

//#region 🔖️Artifact
/// 🧬️ Full writer artifact across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub language_id: String,
    #[state(artifact)]
    pub uri: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.document")]
    pub document: WriterDocumentChild,
    #[state(presence)]
    pub editor_selection: Option<WriterEditorSelection>,
    #[state(presence)]
    pub editor_settings: WriterEditorSettings,
    #[state(config)]
    pub format_signal: u32,
    #[state(config)]
    pub lint_signal: u32,
    #[state(config)]
    pub revision: u32,
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera_x: f64,
    #[state(config)]
    pub camera_y: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for WriterArtifact {
    fn default() -> Self {
        Self::from_snapshot(WriterSnapshot::default())
    }
}

impl WriterArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> WriterSnapshot {
        WriterSnapshot { schema: self.schema.clone(), id: self.id.clone(), language_id: self.language_id.clone(), uri: self.uri.clone(), document: self.document.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot with UI defaults.
    pub async fn from_snapshot(snapshot: WriterSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, language_id: snapshot.language_id, uri: snapshot.uri, document: snapshot.document, ..Self::default_ui() }
    }

    async fn default_ui() -> Self {
        Self {
            schema: WRITER_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            language_id: "plaintext".into(),
            uri: crate::artifacts::writer::default_uri(),
            document: document_child_handle_and_cache("", "", "plaintext"),
            editor_selection: None,
            editor_settings: WriterEditorSettings::default(),
            format_signal: 0,
            lint_signal: 0,
            revision: 0,
            engagement_input: String::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot.
    pub async fn set_snapshot(&mut self, snapshot: WriterSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.language_id = snapshot.language_id;
        self.uri = snapshot.uri;
        self.document = snapshot.document;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.writer.writer` — twenty handcrafted schema leaves.
pub async fn writer_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.writer.writer",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers
/// 🌱️ The canonical empty `WriterSnapshot` — every artifact-tree helper here that needs a fallback or a
/// baseline document builds off this one value.
pub async fn empty_writer_snapshot() -> WriterSnapshot {
    WriterSnapshot { schema: WRITER_DOCUMENT_SCHEMA.into(), id: "empty".into(), language_id: "plaintext".into(), uri: "writer://empty".into(), document: document_child_handle_and_cache("empty", "", "plaintext") }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Languages
/// 🎨️ One classified span of source text — editor scene semantic-token payload shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrammarToken {
    pub class: String,
    pub start: usize,
    pub end: usize,
}

async fn byte_span_to_text_span(text: &str, start: usize, end: usize) -> dsl::TextSpan {
    let safe_end = end.min(text.len());
    let safe_start = start.min(safe_end);
    let prefix = &text[..safe_start];
    let line = prefix.chars().filter(|&c| c == '\n').count() as u32 + 1;
    let column = prefix.rfind('\n').map(|i| safe_start - i).unwrap_or(safe_start) as u32;
    let length = (safe_end - safe_start) as u32;
    dsl::TextSpan::with_length(line, column, length.max(1))
}

async fn token_class_from_name(name: &str) -> dsl::TokenClass {
    match name {
        "keyword" => dsl::TokenClass::Keyword,
        "string" => dsl::TokenClass::String,
        "number" => dsl::TokenClass::Number,
        "operator" => dsl::TokenClass::Operator,
        "comment" => dsl::TokenClass::Comment,
        "error" => dsl::TokenClass::Error,
        _ => dsl::TokenClass::Ident,
    }
}

/// 🎼️ The `jack` query-language `dsl::DslIdiom`, used directly by writer's jack completion surface.
pub(crate) struct JackWriterIdiom;

impl dsl::DslIdiom for JackWriterIdiom {
    const LANG: &'static str = "jack";
    type Ast = String;

    async fn parse(text: &str) -> Result<Self::Ast, dsl::TextError> {
        trinity::core::format(text).map_err(|e| dsl::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }

    async fn print(ast: &Self::Ast) -> String {
        ast.clone()
    }

    async fn classify(text: &str) -> Vec<(dsl::TokenClass, dsl::TextSpan)> {
        trinity::core::semantic_tokens(text).into_iter().map(|t| (token_class_from_name(&t.class), byte_span_to_text_span(text, t.start, t.end))).collect()
    }

    async fn complete(text: &str, offset: usize) -> Vec<dsl::CompletionItem> {
        let graph = trinity::core::example_graph();
        trinity::core::complete(&graph, text, offset).into_iter().map(|item| dsl::CompletionItem { label: item.label, detail: item.detail }).collect()
    }
}

/// 🎼️ The `wire` protocol-text `dsl::DslIdiom` — see [`JackWriterIdiom`]'s doc comment for why it lives here.
pub(crate) struct WireWriterIdiom;

impl dsl::DslIdiom for WireWriterIdiom {
    const LANG: &'static str = "wire";
    type Ast = String;

    async fn parse(text: &str) -> Result<Self::Ast, dsl::TextError> {
        dsl::parse_wire_text(text.trim())?;
        Ok(text.to_string())
    }

    async fn print(ast: &Self::Ast) -> String {
        ast.clone()
    }

    async fn classify(text: &str) -> Vec<(dsl::TokenClass, dsl::TextSpan)> {
        let limits = dsl::Limits::default();
        let Ok(tokens) = dsl::lex(text, &limits, false) else {
            return Vec::new();
        };
        tokens
            .into_iter()
            .filter(|t| !t.kind.is_trivia() && t.kind != dsl::TokenKind::Eof)
            .map(|t| {
                let class = match t.kind {
                    dsl::TokenKind::Arrow | dsl::TokenKind::DashArrow | dsl::TokenKind::EdgeArrow | dsl::TokenKind::BackArrow => dsl::TokenClass::Operator,
                    dsl::TokenKind::Float | dsl::TokenKind::Int => dsl::TokenClass::Number,
                    dsl::TokenKind::Text => dsl::TokenClass::String,
                    dsl::TokenKind::Ident => dsl::TokenClass::Ident,
                    _ => dsl::TokenClass::Punctuation,
                };
                let start = t.byte_range.0 as usize;
                let end = t.byte_range.1 as usize;
                (class, byte_span_to_text_span(text, start, end))
            })
            .collect()
    }
}

/// @emoji 🎨️ Classifies `text` through the language registry (`idiom` / `LanguageSpec` hooks).
pub async fn tokenize_language(text: &str, language_id: &str) -> Vec<GrammarToken> {
    if language_id == "jack" {
        return trinity::core::semantic_tokens(text).into_iter().map(|t| GrammarToken { class: t.class, start: t.start, end: t.end }).collect();
    }
    if language_id == "wire" {
        let limits = dsl::Limits::default();
        if let Ok(tokens) = dsl::lex(text, &limits, false) {
            return tokens
                .into_iter()
                .filter(|t| !t.kind.is_trivia() && t.kind != dsl::TokenKind::Eof)
                .map(|t| {
                    let class = match t.kind {
                        dsl::TokenKind::Arrow | dsl::TokenKind::DashArrow | dsl::TokenKind::EdgeArrow | dsl::TokenKind::BackArrow => "operator",
                        dsl::TokenKind::Float | dsl::TokenKind::Int => "number",
                        dsl::TokenKind::Text => "string",
                        dsl::TokenKind::Ident => "ident",
                        _ => "punctuation",
                    };
                    GrammarToken { class: class.into(), start: t.byte_range.0 as usize, end: t.byte_range.1 as usize }
                })
                .collect();
        }
    }
    if let Some(hooks) = dsl::idiom(language_id) {
        return (hooks.classify)(text)
            .into_iter()
            .enumerate()
            .map(|(i, (class, _span))| GrammarToken { class: format!("{class:?}").trim_start_matches("TokenClass::").to_ascii_lowercase(), start: i, end: i.saturating_add(1).min(text.len()) })
            .collect();
    }
    Vec::new()
}

pub async fn language_completions_json(text: &str, language_id: &str, cursor: usize) -> Option<String> {
    if let Some(spec) = dsl::language(language_id) {
        let session = dsl::lsp::LanguageSession::open(spec, text.to_string());
        let items: Vec<Value> = session.completions_at(cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
        return serde_json::to_string(&items).ok();
    }
    if let Some(hooks) = dsl::idiom(language_id) {
        let items: Vec<Value> = (hooks.complete)(text, cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
        return serde_json::to_string(&items).ok();
    }
    None
}

pub async fn jack_completions_json(text: &str, cursor: usize) -> Option<String> {
    let items: Vec<Value> = <JackWriterIdiom as dsl::DslIdiom>::complete(text, cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
    serde_json::to_string(&items).ok()
}

/// 🎼️ `wire` counterpart of [`jack_completions_json`] — see [`WireWriterIdiom`]'s doc comment.
pub async fn wire_completions_json(text: &str, cursor: usize) -> Option<String> {
    let items: Vec<Value> = <WireWriterIdiom as dsl::DslIdiom>::complete(text, cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
    serde_json::to_string(&items).ok()
}

/// 🪞️ Canonical jack format when possible, else a whitespace-only normalization for other languages.
pub fn format_writer_text(text: &str, language_id: &str) -> String {
    if language_id == "jack" {
        if let Ok(formatted) = trinity::core::format(text) {
            return formatted;
        }
    }
    let mut normalized: String = text.lines().map(|line| line.trim_end()).collect::<Vec<_>>().join("\n");
    if !text.is_empty() && !normalized.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}
//#endregion 🔖️Languages

//#region 🔖️JackAst
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

pub async fn jack_ast_tree_icon(kind: &str) -> Option<&'static str> {
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

/// 🌉️ Adapts trinity::core's shared [`trinity::core::SpannedNode`] tree into writer's own [`JackAstNode`]
/// (adds the stable tree-item `id` the outline panel needs; `kind`/`label`/spans pass through unchanged).
async fn jack_ast_from_spanned(node: &trinity::core::SpannedNode) -> JackAstNode {
    JackAstNode { id: format!("jack-ast-{}-{}-{}", node.kind, node.start, node.end), kind: node.kind.clone(), label: node.label.clone(), start: node.start, end: node.end, children: node.children.iter().map(jack_ast_from_spanned).collect() }
}

/// 🌳️ Parse jack source into a span-tracked AST for hierarchy panels, via the shared `trinity::core` parser.
pub async fn parse_jack_ast(text: &str) -> JackAstNode {
    jack_ast_from_spanned(&trinity::core::parse_spanned(text))
}

/// 🎯️ Deepest AST node containing a byte offset.
pub async fn find_deepest_jack_ast_node_at(root: &JackAstNode, offset: usize) -> Option<&JackAstNode> {
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
pub async fn jack_ast_node_by_id<'a>(root: &'a JackAstNode, id: &str) -> Option<&'a JackAstNode> {
    if root.id == id {
        return Some(root);
    }
    root.children.iter().find_map(|child| jack_ast_node_by_id(child, id))
}

/// 🖱️ Smallest AST node that fully contains a selection range.
pub async fn jack_ast_node_for_selection(root: &JackAstNode, start: usize, end: usize) -> Option<&JackAstNode> {
    async fn visit<'a>(node: &'a JackAstNode, start: usize, end: usize, best: &mut Option<&'a JackAstNode>) {
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
pub async fn selectable_spans_for_jack(text: &str, tokens: &[GrammarToken]) -> Vec<SelectableSpan> {
    let mut spans: Vec<SelectableSpan> = tokens.iter().map(|token| SelectableSpan { start: token.start, end: token.end, kind: "atomic".into(), head_end: None, tail_start: None }).collect();
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

async fn jack_placeholder_visible(caret: usize, offset: usize) -> bool {
    let caret = caret as i64;
    let offset = offset as i64;
    caret >= offset - 32 && caret <= offset + 48
}

/// 🔤️ Fine-grained, never-fails jack tokens for editor heuristics — routed through `trinity::core`'s shared
/// forgiving lexer instead of a hand-rolled writer copy.
fn jack_tokens(text: &str) -> Vec<SpannedToken> {
    lex_spanned(text, true).unwrap_or_default()
}

async fn jack_token_expects_expr(token: &Token) -> bool {
    matches!(token, Token::And | Token::Or)
}

async fn jack_token_expects_pattern(token: &Token) -> bool {
    matches!(token, Token::KwMatch | Token::KwCreate | Token::KwMerge)
}

/// 👻️ Required jack token placeholders near the caret (premigration `jackEditorPlaceholders`).
pub async fn jack_editor_placeholders(text: &str, caret: usize) -> Vec<JackEditorPlaceholder> {
    use Token;
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

const JACK_NEWLINE_AFTER_KEYWORDS: &[Token] = &[Token::KwMatch, Token::KwWhere, Token::KwReturn, Token::KwCreate, Token::KwDelete, Token::KwSet, Token::KwMerge, Token::And, Token::Or];

async fn jack_lex_token_at_offset(tokens: &[SpannedToken], offset: usize) -> Option<&SpannedToken> {
    for token in tokens {
        if token.token == Token::Eof {
            break;
        }
        if offset >= token.start && offset <= token.end {
            return Some(token);
        }
    }
    None
}

/// ↩️ Whether a jack query may break onto a new line at a byte offset (premigration `jackNewlineAllowedAt`).
pub async fn jack_newline_allowed_at(text: &str, offset: usize) -> bool {
    use Token;
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
    if before.trim_end().chars().last().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') && after.trim_start().starts_with('.') {
        return false;
    }
    if before.trim_end().ends_with(':') && after.trim_start().chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
        return false;
    }
    if before.trim_end().ends_with('.') && after.trim_start().chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
        return false;
    }

    let mut prev: Option<&SpannedToken> = None;
    let mut next: Option<&SpannedToken> = None;
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
            return next.is_none_or(|n| n.token != Token::Dot);
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
pub async fn jack_newline_gate_offsets(text: &str) -> Vec<usize> {
    (0..=text.len()).filter(|&offset| text.is_char_boundary(offset) && jack_newline_allowed_at(text, offset)).collect()
}

/// 🔗️ Bound jack variable names from pattern bindings (premigration `jackBoundVariableNames`).
pub fn jack_bound_variable_names(text: &str) -> std::collections::HashSet<String> {
    use Token;
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

fn is_jack_variable_use_token(tokens: &[SpannedToken], index: usize, bound: &std::collections::HashSet<String>) -> bool {
    use Token;
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
    use Token;
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
    use Token;
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
    sorted.sort_by_key(|a| std::cmp::Reverse(a.0));
    let mut out = text.to_string();
    for (start, end) in sorted {
        if start <= end && end <= out.len() {
            out.replace_range(start..end, new_name);
        }
    }
    out
}
//#endregion 🔖️JackEditor

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const CANONICAL_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";

    #[semio_framework_async_macros::async_test]
    async fn parses_full_jack_ast_shape() {
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

    #[semio_framework_async_macros::async_test]
    async fn selection_maps_to_smallest_containing_ast_node() {
        let root = parse_jack_ast(CANONICAL_QUERY);
        let a_offset = CANONICAL_QUERY.find("a:Piece").unwrap();
        let node = jack_ast_node_for_selection(&root, a_offset, a_offset + 1).expect("node");
        assert_eq!(node.kind, "var");
    }

    #[semio_framework_async_macros::async_test]
    async fn selectable_spans_include_var_label_and_property_access() {
        let text = "MATCH (a1:Piece) RETURN a1.name";
        let tokens = tokenize_language(text, "jack");
        let spans = selectable_spans_for_jack(text, &tokens);
        assert!(spans.iter().any(|s| s.kind == "varLabel" && s.start == 7 && s.end == 15));
        assert!(spans.iter().any(|s| s.kind == "propertyAccess" && s.start == 24 && s.end == 31));
    }

    #[semio_framework_async_macros::async_test]
    async fn symbol_occurrences_find_bound_variable_uses() {
        let symbol = jack_symbol_at_offset(CANONICAL_QUERY, CANONICAL_QUERY.find("a.name").unwrap()).expect("symbol");
        assert_eq!(symbol.kind, JackSymbolKind::Variable);
        assert_eq!(symbol.occurrences.len(), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn symbol_at_label_position_is_node_kind_not_variable() {
        let symbol = jack_symbol_at_offset(CANONICAL_QUERY, CANONICAL_QUERY.find("Piece").unwrap() + 1).expect("symbol");
        assert_eq!(symbol.kind, JackSymbolKind::NodeKind);
    }

    #[semio_framework_async_macros::async_test]
    async fn placeholders_suggest_expr_after_and() {
        let text = "MATCH (a:Piece) WHERE a.name = 'x' AND ";
        let placeholders = jack_editor_placeholders(text, text.len());
        assert!(placeholders.iter().any(|p| p.label == "expr"));
    }

    #[semio_framework_async_macros::async_test]
    async fn placeholders_suggest_label_after_colon() {
        let text = "MATCH (a:";
        let placeholders = jack_editor_placeholders(text, text.len());
        assert!(placeholders.iter().any(|p| p.label == "Label"));
    }

    #[semio_framework_async_macros::async_test]
    async fn newline_gates_allow_after_match_close_paren() {
        let text = "MATCH (a:Piece)";
        let gates = jack_newline_gate_offsets(text);
        assert!(gates.contains(&text.len()));
    }

    #[semio_framework_async_macros::async_test]
    async fn newline_gates_disallow_inside_token() {
        let text = "MATCH (a:Piece)";
        let inside = text.find("Piece").unwrap() + 2;
        assert!(!jack_newline_allowed_at(text, inside));
    }

    #[semio_framework_async_macros::async_test]
    async fn newline_gates_disallow_before_dot() {
        let text = "RETURN a.name";
        let before_dot = text.find('.').unwrap();
        assert!(!jack_newline_allowed_at(text, before_dot));
    }
}
//#endregion 🧪️Tests
