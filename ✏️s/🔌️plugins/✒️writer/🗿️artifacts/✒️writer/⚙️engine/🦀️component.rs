//! ⚙️ Writer artifact — headless compute over the `WriterProjection` projection (constitutional: engine).
//!
//! Everything here is pure over `WriterProjection`/`WriterCamera` (artifact types) and `trinity::core`'s
//! shared parser. It deliberately takes no `crate::apps::writer::config::WriterConfig` parameter: that
//! type lives at APP level (view state, not document — artifacts must never depend on apps). A helper
//! that needs `WriterConfig` (e.g. `editor_hover_context`, shared by the main window and the document
//! panel) lives in `crate::apps::writer` instead, even though it has more than one consumer — see that
//! function's doc comment. The rule for what DOES land here: a pure `WriterProjection`-only helper with
//! more than one consumer across the taxonomy tree lives here; one with exactly one consumer lives in
//! that consumer's own component file.

use super::{WriterProjection, WRITER_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use trinity::core::{example_graph, lint, Diagnostic};

//#region 🔖️GrammarToken
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrammarToken {
    pub class: String,
    pub start: usize,
    pub end: usize,
}
//#endregion 🔖️GrammarToken

//#region 🔖️Register
/// 🗂️ Registers `WriterProjection`'s pack↔dsl codec under `WRITER_DOCUMENT_SCHEMA` so `framework/sync`'s
/// folder endpoints and any other schema-string-keyed caller can print/parse writer documents. Called
/// from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    register_writer_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::writer::WriterPlayApp>(WRITER_DOCUMENT_SCHEMA);
}

//#region 🔖️Languages
fn byte_span_to_text_span(text: &str, start: usize, end: usize) -> dsl::TextSpan {
    let safe_end = end.min(text.len());
    let safe_start = start.min(safe_end);
    let prefix = &text[..safe_start];
    let line = prefix.chars().filter(|&c| c == '\n').count() as u32 + 1;
    let column = prefix.rfind('\n').map(|i| safe_start - i).unwrap_or(safe_start) as u32;
    let length = (safe_end - safe_start) as u32;
    dsl::TextSpan::with_length(line, column, length.max(1))
}

fn token_class_from_name(name: &str) -> dsl::TokenClass {
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

struct JackWriterIdiom;

impl dsl::DslIdiom for JackWriterIdiom {
    const LANG: &'static str = "jack";
    type Ast = String;

    fn parse(text: &str) -> Result<Self::Ast, dsl::TextError> {
        trinity::core::format(text).map_err(|e| dsl::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }

    fn print(ast: &Self::Ast) -> String {
        ast.clone()
    }

    fn classify(text: &str) -> Vec<(dsl::TokenClass, dsl::TextSpan)> {
        trinity::core::semantic_tokens(text)
            .into_iter()
            .map(|t| (token_class_from_name(&t.class), byte_span_to_text_span(text, t.start, t.end)))
            .collect()
    }

    fn complete(text: &str, offset: usize) -> Vec<dsl::CompletionItem> {
        let graph = trinity::core::example_graph();
        trinity::core::complete(&graph, text, offset)
            .into_iter()
            .map(|item| dsl::CompletionItem { label: item.label, detail: item.detail })
            .collect()
    }
}

struct WireWriterIdiom;

impl dsl::DslIdiom for WireWriterIdiom {
    const LANG: &'static str = "wire";
    type Ast = String;

    fn parse(text: &str) -> Result<Self::Ast, dsl::TextError> {
        dsl::parse_wire_text(text.trim())?;
        Ok(text.to_string())
    }

    fn print(ast: &Self::Ast) -> String {
        ast.clone()
    }

    fn classify(text: &str) -> Vec<(dsl::TokenClass, dsl::TextSpan)> {
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




fn register_writer_languages() {
    dsl::register_idiom(dsl::hooks_for::<JackWriterIdiom>());
    dsl::register_idiom(dsl::hooks_for::<WireWriterIdiom>());
    let jack_hooks = dsl::hooks_for::<JackWriterIdiom>();
    dsl::register_language(dsl::LanguageSpec {
        id: "jack",
        extension: None,
        role: dsl::LanguageRole::Embedded,
        grammar: None,
        grammar_path: None,
        protocol: None,
        protocol_path: None,
        hooks: jack_hooks,
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.document",
        extension: Some("writer"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::writer::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::writer::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::writer::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::writer::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("writer.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::writer::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::writer::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::writer::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::writer::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("writer.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::writer::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::writer::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("writer.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::writer::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::writer::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("writer.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "writer.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::writer::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::writer::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("writer.spr"),
    });
}

/// @emoji 🎨️ Classifies `text` through the language registry (`idiom` / `LanguageSpec` hooks).
pub fn tokenize_language(text: &str, language_id: &str) -> Vec<GrammarToken> {
    if language_id == "jack" {
        return trinity::core::semantic_tokens(text)
            .into_iter()
            .map(|t| GrammarToken { class: t.class, start: t.start, end: t.end })
            .collect();
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
            .map(|(i, (class, _span))| GrammarToken {
                class: format!("{class:?}").trim_start_matches("TokenClass::").to_ascii_lowercase(),
                start: i,
                end: i.saturating_add(1).min(text.len()),
            })
            .collect();
    }
    Vec::new()
}

/// @emoji 📡️ Semantic token payload for the text editor scene (LSP `data` array or grammar tokens).
pub fn language_tokens_json(document: &WriterProjection) -> Option<String> {
    eprintln!(
        "[DEBUG] writer.engine language_tokens_json language_id={} text_len={}",
        document.language_id,
        document.text.len()
    );
    if let Some(spec) = dsl::language(&document.language_id) {
        let session = dsl_lsp::LanguageSession::open(spec, document.text.clone());
        return serde_json::to_string(&session.semantic_tokens_lsp()).ok();
    }
    if dsl::idiom(&document.language_id).is_some() {
        let tokens = tokenize_language(&document.text, &document.language_id);
        return serde_json::to_string(&tokens).ok();
    }
    None
}
//#endregion 🔖️Languages
//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports plus one
/// extra output, `text:out` (Text×Document, kind `text.document`, `Many` — a workflow may fan this
/// writer's text out to several consumers, e.g. `playbook`'s `chapters:in`).
pub fn writer_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: WRITER_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Text, form: semio_framework_plugin::MediaForm::Document },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "text:out".into(),
            label: "Text".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Text, form: semio_framework_plugin::MediaForm::Document },
            kind_id: Some("text.document".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "text.document".into(), name: "Text Document".into(), dimension: "text".into(), component_kind: "writer".into() },
    }
}

/// 📤️ The JSON shape `"text:out"` exports and `playbook`'s `"chapters:in"` imports — one writer
/// document's text as one "chapter" (`title` mirrors the document id, `language_id` lets an importer
/// route jack/wire content differently from prose if it ever wants to).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterChapterPayload {
    pub id: String,
    pub title: String,
    pub text: String,
    pub language_id: String,
}

/// 🎞️ Projects a `WriterProjection` onto the `"text:out"` chapter payload shape.
pub fn writer_chapter_payload(document: &WriterProjection) -> WriterChapterPayload {
    WriterChapterPayload { id: document.id.clone(), title: document.id.clone(), text: document.text.clone(), language_id: document.language_id.clone() }
}
//#endregion 🔖️Io

//#region 🔖️Examples
/// 📄️ The `jack` example, parsed once from {@link crate::artifacts::writer::dsl::JACK_EXAMPLE_TEXT} —
/// the source of truth for every call site below (`setActiveExample`, `.example("jack", ...)`, tests,
/// "file-text"); never re-embed the raw text.
pub fn jack_example_document() -> WriterProjection {
    <WriterProjection as store::DocumentDsl>::parse_dsl(crate::artifacts::writer::dsl::JACK_EXAMPLE_TEXT).unwrap_or_else(|_| empty_writer_projection())
}

/// 📄️ JSON re-serialization of {@link jack_example_document}, for the framework-generic call sites
/// (`.example(...)`, `render(...)`) that still take a document as a JSON string.
pub fn jack_example_json() -> String {
    serde_json::to_string(&jack_example_document()).expect("serialize jack example document")
}

/// 📄️ The `dag.jack` example, parsed once from {@link crate::artifacts::writer::dsl::DAG_JACK_EXAMPLE_TEXT}
/// — see {@link jack_example_document}.
pub fn dag_jack_example_document() -> WriterProjection {
    <WriterProjection as store::DocumentDsl>::parse_dsl(crate::artifacts::writer::dsl::DAG_JACK_EXAMPLE_TEXT).unwrap_or_else(|_| empty_writer_projection())
}

/// 📄️ JSON re-serialization of {@link dag_jack_example_document} — see {@link jack_example_json}.
pub fn dag_jack_example_json() -> String {
    serde_json::to_string(&dag_jack_example_document()).expect("serialize dag.jack example document")
}
//#endregion 🔖️Examples

//#region 🔖️DocumentHelpers
pub fn empty_writer_projection() -> WriterProjection {
    WriterProjection { schema: WRITER_DOCUMENT_SCHEMA.into(), id: "empty".into(), language_id: "plaintext".into(), uri: "writer://empty".into(), text: String::new() }
}

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

/// 🌉️ Adapts trinity::core's shared [`trinity::core::SpannedNode`] tree into writer's own [`JackAstNode`]
/// (adds the stable tree-item `id` the outline panel needs; `kind`/`label`/spans pass through unchanged).
fn jack_ast_from_spanned(node: &trinity::core::SpannedNode) -> JackAstNode {
    JackAstNode { id: format!("jack-ast-{}-{}-{}", node.kind, node.start, node.end), kind: node.kind.clone(), label: node.label.clone(), start: node.start, end: node.end, children: node.children.iter().map(jack_ast_from_spanned).collect() }
}

/// 🌳️ Parse jack source into a span-tracked AST for hierarchy panels, via the shared `trinity::core` parser.
pub fn parse_jack_ast(text: &str) -> JackAstNode {
    jack_ast_from_spanned(&trinity::core::parse_spanned(text))
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
pub fn jack_ast_node_for_selection(root: &JackAstNode, start: usize, end: usize) -> Option<&JackAstNode> {
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

fn jack_placeholder_visible(caret: usize, offset: usize) -> bool {
    let caret = caret as i64;
    let offset = offset as i64;
    caret >= offset - 32 && caret <= offset + 48
}

/// 🔤️ Fine-grained, never-fails jack tokens for editor heuristics — routed through `trinity::core`'s shared
/// forgiving lexer instead of a hand-rolled writer copy.
fn jack_tokens(text: &str) -> Vec<trinity::core::SpannedToken> {
    trinity::core::lex_spanned(text, true).unwrap_or_default()
}

fn jack_token_expects_expr(token: &trinity::core::Token) -> bool {
    matches!(token, trinity::core::Token::And | trinity::core::Token::Or)
}

fn jack_token_expects_pattern(token: &trinity::core::Token) -> bool {
    matches!(token, trinity::core::Token::KwMatch | trinity::core::Token::KwCreate | trinity::core::Token::KwMerge)
}

/// 👻️ Required jack token placeholders near the caret (premigration `jackEditorPlaceholders`).
pub fn jack_editor_placeholders(text: &str, caret: usize) -> Vec<JackEditorPlaceholder> {
    use trinity::core::Token;
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

const JACK_NEWLINE_AFTER_KEYWORDS: &[trinity::core::Token] = &[
    trinity::core::Token::KwMatch,
    trinity::core::Token::KwWhere,
    trinity::core::Token::KwReturn,
    trinity::core::Token::KwCreate,
    trinity::core::Token::KwDelete,
    trinity::core::Token::KwSet,
    trinity::core::Token::KwMerge,
    trinity::core::Token::And,
    trinity::core::Token::Or,
];

fn jack_lex_token_at_offset(tokens: &[trinity::core::SpannedToken], offset: usize) -> Option<&trinity::core::SpannedToken> {
    for token in tokens {
        if token.token == trinity::core::Token::Eof {
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
    use trinity::core::Token;
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

    let mut prev: Option<&trinity::core::SpannedToken> = None;
    let mut next: Option<&trinity::core::SpannedToken> = None;
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
pub fn jack_newline_gate_offsets(text: &str) -> Vec<usize> {
    (0..=text.len()).filter(|&offset| text.is_char_boundary(offset) && jack_newline_allowed_at(text, offset)).collect()
}

/// 🔗️ Bound jack variable names from pattern bindings (premigration `jackBoundVariableNames`).
pub fn jack_bound_variable_names(text: &str) -> std::collections::HashSet<String> {
    use trinity::core::Token;
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

fn is_jack_variable_use_token(tokens: &[trinity::core::SpannedToken], index: usize, bound: &std::collections::HashSet<String>) -> bool {
    use trinity::core::Token;
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
    use trinity::core::Token;
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
    use trinity::core::Token;
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

pub fn language_completions_json(text: &str, language_id: &str, cursor: usize) -> Option<String> {
    if let Some(spec) = dsl::language(language_id) {
        let session = dsl_lsp::LanguageSession::open(spec, text.to_string());
        let items: Vec<Value> = session
            .completions_at(cursor)
            .into_iter()
            .map(|item| json!({ "label": item.label, "detail": item.detail }))
            .collect();
        return serde_json::to_string(&items).ok();
    }
    if let Some(hooks) = dsl::idiom(language_id) {
        let items: Vec<Value> = (hooks.complete)(text, cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
        return serde_json::to_string(&items).ok();
    }
    None
}

pub fn language_diagnostics_json(document: &WriterProjection, lint_signal: u32) -> Option<String> {
    if document.language_id == "jack" {
        let graph = example_graph();
        let diagnostics: Vec<Value> = lint(&graph, &document.text)
            .into_iter()
            .map(|diag: Diagnostic| json!({ "start": diag.start, "end": diag.end, "severity": diag.severity, "message": diag.message }))
            .collect();
        return serde_json::to_string(&diagnostics).ok();
    }
    if let Some(hooks) = dsl::idiom(&document.language_id) {
        if let Err(err) = (hooks.canonicalize)(&document.text) {
            let end = document.text.len().max(1);
            return serde_json::to_string(&[json!({ "start": 0, "end": end, "severity": "error", "message": err.message })]).ok();
        }
    } else if let Some(spec) = dsl::language(&document.language_id) {
        let session = dsl_lsp::LanguageSession::open(spec, document.text.clone());
        if let Err(err) = session.canonicalize() {
            let end = document.text.len().max(1);
            return serde_json::to_string(&[json!({ "start": 0, "end": end, "severity": "error", "message": err.message })]).ok();
        }
    }
    if lint_signal > 0 {
        return Some(
            json!([{ "start": 0, "end": document.text.len().max(1), "severity": "info", "message": format!("Lint pass #{lint_signal}") }])
                .to_string(),
        );
    }
    None
}

pub fn jack_completions_json(text: &str, cursor: usize) -> Option<String> {
    language_completions_json(text, "jack", cursor)
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

//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent writer artifact engine.
pub struct WriterEngine {
    projection: WriterProjection,
}

impl WriterEngine {
    pub fn new(projection: WriterProjection) -> Self {
        Self { projection }
    }

    pub fn into_projection(self) -> WriterProjection {
        self.projection
    }
}

impl protocol::ArtifactEngine for WriterEngine {
    type Projection = WriterProjection;
    type Mutation = crate::artifacts::writer::mutations::WriterMutation;
    type Diff = crate::artifacts::writer::diff::WriterDiff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::writer::mutations::apply_writer_mutation(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
