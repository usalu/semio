//! 🧬️ Writer artifact schema — every field with its state class.

use crate::{document_child_handle_with_text, WriterDocumentChild, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::lexer::{lex_spanned, SpannedToken, Token};
use serde::{Deserialize, Serialize};
//#region 🔖️Artifact
/// 🧬️ Full writer artifact across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
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
    /// ✍️ See `WriterSnapshot::text` — the persisted payload the composed `document` child is
    /// derived from.
    #[state(artifact)]
    pub text: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub document: WriterDocumentChild,
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
    pub fn to_snapshot(&self) -> WriterSnapshot {
        WriterSnapshot { schema: self.schema.clone(), id: self.id.clone(), language_id: self.language_id.clone(), uri: self.uri.clone(), text: self.text.clone(), document: self.document.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot with UI defaults.
    pub fn from_snapshot(snapshot: WriterSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, language_id: snapshot.language_id, uri: snapshot.uri, text: snapshot.text, document: snapshot.document, ..Self::default_ui() }
    }

    fn default_ui() -> Self {
        Self { schema: WRITER_DOCUMENT_SCHEMA.into(), id: String::new(), language_id: "plaintext".into(), uri: crate::default_uri(), text: String::new(), document: document_child_handle_with_text("", "", "plaintext") }
    }

    /// 🔄 Writes persistent fields from a snapshot.
    pub fn set_snapshot(&mut self, snapshot: WriterSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.language_id = snapshot.language_id;
        self.uri = snapshot.uri;
        self.text = snapshot.text;
        self.document = snapshot.document;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.writer.writer` — twenty handcrafted schema leaves.
pub fn writer_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.writer.writer", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.writer.writer",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: "",
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: "",
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: "",
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️DocumentHelpers
/// 🌱️ The canonical empty `WriterSnapshot` — every artifact-tree helper here that needs a fallback or a
/// baseline document builds off this one value.
pub fn empty_writer_snapshot() -> WriterSnapshot {
    WriterSnapshot { schema: WRITER_DOCUMENT_SCHEMA.into(), id: "empty".into(), language_id: "plaintext".into(), uri: "writer://empty".into(), text: String::new(), document: document_child_handle_with_text("empty", "", "plaintext") }
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









/// 🎨️ Classifies `text` through the language registry (`idiom` / `LanguageSpec` hooks).
pub fn tokenize_language(text: &str, language_id: &str) -> Vec<GrammarToken> {
    if language_id == "jack" {
        return semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::language_service::semantic_tokens(text).into_iter().map(|t| GrammarToken { class: t.class, start: t.start, end: t.end }).collect();
    }
    if language_id == "wire" {
        let limits = semio_framework_diagnostic::Limits::default();
        if let Ok(tokens) = semio_framework_dsl::lex(text, &limits, false) {
            return tokens
                .into_iter()
                .filter(|t| !t.kind.is_trivia() && t.kind != semio_framework_dsl::TokenKind::Eof)
                .map(|t| {
                    let class = match t.kind {
                        semio_framework_dsl::TokenKind::Arrow | semio_framework_dsl::TokenKind::DashArrow | semio_framework_dsl::TokenKind::EdgeArrow | semio_framework_dsl::TokenKind::BackArrow => "operator",
                        semio_framework_dsl::TokenKind::Float | semio_framework_dsl::TokenKind::Int => "number",
                        semio_framework_dsl::TokenKind::Text => "string",
                        semio_framework_dsl::TokenKind::Ident => "ident",
                        _ => "punctuation",
                    };
                    GrammarToken { class: class.into(), start: t.byte_range.0 as usize, end: t.byte_range.1 as usize }
                })
                .collect();
        }
    }
    if let Some(hooks) = semio_framework_dsl::idiom(language_id) {
        return (hooks.classify)(text)
            .into_iter()
            .enumerate()
            .map(|(i, (class, _span))| GrammarToken { class: format!("{class:?}").trim_start_matches("TokenClass::").to_ascii_lowercase(), start: i, end: i.saturating_add(1).min(text.len()) })
            .collect();
    }
    Vec::new()
}







/// 🪞️ Canonical jack format when possible, else a whitespace-only normalization for other languages.
pub fn format_writer_text(text: &str, language_id: &str) -> String {
    if language_id == "jack" {
        if let Ok(formatted) = semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::language_service::format(text) {
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
fn jack_ast_from_spanned(node: &semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::language_service::SpannedNode) -> JackAstNode {
    JackAstNode { id: format!("jack-ast-{}-{}-{}", node.kind, node.start, node.end), kind: node.kind.clone(), label: node.label.clone(), start: node.start, end: node.end, children: node.children.iter().map(jack_ast_from_spanned).collect() }
}

/// 🌳️ Parse jack source into a span-tracked AST for hierarchy panels, via the shared `trinity::core` parser.
pub fn parse_jack_ast(text: &str) -> JackAstNode {
    jack_ast_from_spanned(&semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::language_service::parse_spanned(text))
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
fn jack_tokens(text: &str) -> Vec<SpannedToken> {
    lex_spanned(text, true).unwrap_or_default()
}

fn jack_token_expects_expr(token: &Token) -> bool {
    matches!(token, Token::And | Token::Or)
}

fn jack_token_expects_pattern(token: &Token) -> bool {
    matches!(token, Token::KwMatch | Token::KwCreate | Token::KwMerge)
}

/// 👻️ Required jack token placeholders near the caret (premigration `jackEditorPlaceholders`).
pub fn jack_editor_placeholders(text: &str, caret: usize) -> Vec<JackEditorPlaceholder> {
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

fn jack_lex_token_at_offset(tokens: &[SpannedToken], offset: usize) -> Option<&SpannedToken> {
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
pub fn jack_newline_allowed_at(text: &str, offset: usize) -> bool {
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
pub fn jack_newline_gate_offsets(text: &str) -> Vec<usize> {
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

//#endregion 🧪️Tests

//#region 🧩️ArtifactItems
pub mod artifact_items {
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Constants
pub const WRITER_DOCUMENT_SCHEMA: &str = "writer.document";

/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1's canonical surface
/// coordinate for this artifact — lives at the ARTIFACT level (not under `editor`/`viewer`) so a
/// viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.writer.writer"` matches `definition()`'s own `"s.writer.schema.artifact"`
/// capability row (`descriptor(b"s.writer.writer")`/`claim(schema, "s.writer.writer")` below, not
/// guessed); `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location
/// — i.e. the canonical surface id is `s.writer.writer@1/*#editor` / `s.writer.writer@1/*#viewer`.
pub const WRITER_DIALECT: Dialect = Dialect { artifact_kind: "s.writer.writer", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Constants
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
pub fn default_uri() -> String {
    "writer://empty".into()
}
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use semio_s_artifact_stdio_semio_document::schema::snapshot::{DocBlock, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

//#region 🔖️DocumentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio`/`document` document — writer's
/// authored text now lives in this composed child's block tree rather than inline on
/// `WriterSnapshot` (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, `writer→C:document`).
pub type WriterDocumentChild = store::os_store::artifact_child::ArtifactChild<SemioDocumentSnapshot>;

/// 🌉 REAL bidirectional converter between writer's plain-text/language-id working representation
/// and the composed child's own `SemioDocumentSnapshot` block tree (the "ModelBridge" pattern from
/// `📓️wave3-reports/cad-report.md`) — writer's whole authored body becomes one `DocBlock::Code`
/// leaf (`language` = `language_id`, `text` = the raw buffer), which round-trips losslessly: `Code`
/// carries no run/formatting structure to lose, exactly matching what `text: String` used to carry
/// verbatim. `"plaintext"`/empty language ids map to `None` (no fenced-language hint).
pub fn document_snapshot_from_text(text: &str, language_id: &str) -> SemioDocumentSnapshot {
    let language = (!language_id.is_empty() && language_id != "plaintext").then(|| language_id.to_string());
    SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Vec::new(), images: Vec::new(), blocks: vec![DocBlock::Code { language, text: text.to_string() }] }
}

/// 🌉 Inverse of [`document_snapshot_from_text`] — concatenates every `Code` block's body (the
/// common, lossless case is exactly one); any non-`Code` block is honestly skipped rather than
/// fabricating prose from block content this plugin never authored.
pub fn text_from_document_snapshot(snapshot: &SemioDocumentSnapshot) -> String {
    snapshot
        .blocks
        .iter()
        .filter_map(|block| match block {
            DocBlock::Code { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// 🕸️ Deterministic content-addressed CHILD handle for the document — same `(child_id, target)`
/// for identical `(text, language_id)`, a different pair once the content actually changes; the
/// handle alone is the change signal the parent's diff/mutation machinery reads without ever
/// comparing embedded content, mirroring lowpoly's `mesh_child_handle`/cad's `cad_model_child_handle`.
pub fn document_child_handle(_id: &str, text: &str, language_id: &str) -> WriterDocumentChild {
    let snapshot = document_snapshot_from_text(text, language_id);
    let content_json = semio_framework_pack_json::to_json_string(&snapshot);
    let child_id = store::content_id("document", content_json.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "document".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️DocumentBridge
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{WriterDocumentChild, document_child_handle, document_snapshot_from_text};
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
use std::sync::Arc;
pub use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, artifact-instance working representation of the composed document child's live
/// text. The owner is the `WriterDocumentChild` embedded by one snapshot, so independent apps and
/// stale handles cannot observe or replace one another's text. Persistence and DSL codecs skip
/// the text; constructors and mutations materialize their immutable local owner directly.
pub struct WriterWorkingScene {
    pub text: String,
    pub language_id: String,
}

/// 📝 Materializes text on this exact artifact-child owner without publishing process state.
pub fn attach_writer_document_text(handle: &mut WriterDocumentChild, text: &str) {
    handle.set_local_text(Arc::<str>::from(text));
}

/// 🔎 Reads this child owner's live text, or empty text when the child is unresolved.
pub fn writer_text_for_handle(handle: &WriterDocumentChild) -> String {
    handle.local_text().unwrap_or_default().to_string()
}

/// 🔎 Reads the current document's authored body — the single read call site every
/// render/inference/export path in this plugin uses. The body lives in the PERSISTED
/// `WriterSnapshot::text` payload (see that field's doc comment), not in the composed child handle,
/// so it survives `ArtifactPack`/`ArtifactDsl` and therefore the host's member-less archive door.
pub fn writer_text(snapshot: &WriterSnapshot) -> String {
    snapshot.text.clone()
}

/// 🧵️ Retains the immutable child-text owner for bounded worker jobs without cloning its bytes.
/// The persisted `text` body and the composed `document` handle's local owner are minted together
/// ([`writer_snapshot_with_text`], [`attach_writer_document_text`]), so a body that still agrees with
/// its handle shares that handle's owner; only a body whose handle carries no or other text gets one
/// fresh owner.
pub fn writer_text_owner(snapshot: &WriterSnapshot) -> Arc<str> {
    snapshot.document.local_text_owner().filter(|owner| **owner == *snapshot.text).unwrap_or_else(|| Arc::<str>::from(snapshot.text.as_str()))
}

/// 🏗️ Mints a content-addressed handle carrying its artifact-instance text owner.
pub fn document_child_handle_with_text(id: &str, text: &str, language_id: &str) -> WriterDocumentChild {
    document_child_handle(id, text, language_id).with_local_text(Arc::<str>::from(text))
}

/// 🏗️ Builds a full `WriterSnapshot` from literal text — the standard fixture/import constructor
/// replacing the old 5-field `WriterSnapshot { ..., text }` struct literal now that `document` is a
/// composed child handle, not a plain field.
pub fn writer_snapshot_with_text(schema: &str, id: &str, language_id: &str, uri: &str, text: &str) -> WriterSnapshot {
    WriterSnapshot { schema: schema.into(), id: id.into(), language_id: language_id.into(), uri: uri.into(), text: text.into(), document: document_child_handle_with_text(id, text, language_id) }
}

/// 🌱️ `ArtifactEditor`/`ArtifactViewer::genesis_child_pack` for the composed `document` child — the
/// block tree [`document_snapshot_from_text`] derives from the handle's own text owner. The react
/// shell's `loadDocumentPair` sends `members: []`, so a whole-document load (`setActiveExample` →
/// `Effect::LoadDocument`) derives the slot here; without it the archive closure completes
/// `Incomplete` and the load is refused (`document-archive-replacement.closure-rejected`).
pub fn genesis_writer_child_pack(snapshot: &WriterSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    (slot == "document" && child_id == snapshot.document.child_id).then(|| <SemioDocumentSnapshot as ArtifactPack>::encode_pack(&document_snapshot_from_text(&snapshot.text, &snapshot.language_id)))
}
//#endregion 🔖️WorkingScene
}
//#endregion 🧩️ArtifactItems4
