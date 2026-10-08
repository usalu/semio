//! 📜️ Writer artifact — textual document grammar surface + laws (constitutional: dsl). Owns the
//! REAL `store::ArtifactDsl` impl for `WriterSnapshot` (design.md §1 CORRECTION: the native codec
//! is one bidirectional thing, unsplit, so it lives here rather than mirrored under import/export).

use crate::{schema, WriterDocumentChild, WriterSnapshot};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ `ArtifactDsl` over the derived spec-driven text of `WriterSnapshot::__dsl_spec()`, the same record
/// the pack encodes — the committed `🗣️.dsl.semio` examples are authored in this format.
impl store::ArtifactDsl for WriterSnapshot {
    const EXTENSION: &'static str = "writer";
    fn envelope_id() -> &'static str {
        "writer.writer"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        let mut snapshot = Self::__dsl_from_record(&record)?;
        crate::attach_writer_document_text(&mut snapshot.document, &snapshot.text.clone());
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📄️ The `jack` example document, handcrafted in the `.writer` DSL (see `store::ArtifactDsl`) instead
/// of JSON — {@link jack_example_document}/{@link jack_example_json} are the only ways it should be
/// consumed.
pub const JACK_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
/// 📄️ The `dag.jack` example document, handcrafted in the `.writer` DSL — see {@link JACK_EXAMPLE_TEXT}.
pub const DAG_JACK_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio");



/// 📖️ Parses `.writer` DSL text into a `WriterSnapshot`.
pub fn parse_dsl(text: &str) -> Result<WriterSnapshot, semio_framework_diagnostic::TextError> {
    <WriterSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `WriterSnapshot` back to `.writer` DSL text.
pub fn print_dsl(projection: &WriterSnapshot) -> String {
    store::ArtifactDsl::print_dsl(projection)
}

//#region 🔖️ExternalBridges
/// 📖️ [`parse_dsl`] with a plain-`String` error, reachable from OUTSIDE this crate — `store` is a
/// private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named by the
/// `✒️mutate-writer-1` test adapter that has to read the committed `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_writer_dsl(text: &str) -> Result<WriterSnapshot, String> {
    <WriterSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ [`print_dsl`] under a name an external caller can reach, paired with [`parse_writer_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_writer_dsl(snapshot: &WriterSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges

//#region 🔖️Examples
/// 📄️ The `jack` example, parsed once from {@link JACK_EXAMPLE_TEXT} — the source of truth for every
/// call site below (`setActiveExample`, `.example("jack", ...)`, tests, "file-text"); never re-embed the
/// raw text.
pub fn jack_example_document() -> WriterSnapshot {
    parse_dsl(JACK_EXAMPLE_TEXT).expect("handcrafted current Writer jack example")
}

/// 📄️ JSON re-serialization of {@link jack_example_document}, for the framework-generic call sites
/// (`.example(...)`, `render(...)`) that still take a document as a JSON string.
pub fn jack_example_json() -> String {
    semio_framework_pack_json::to_json_string(&jack_example_document())
}

/// 📄️ The `dag.jack` example, parsed once from {@link DAG_JACK_EXAMPLE_TEXT} — see {@link jack_example_document}.
pub fn dag_jack_example_document() -> WriterSnapshot {
    parse_dsl(DAG_JACK_EXAMPLE_TEXT).expect("handcrafted current Writer DAG example")
}

/// 📄️ JSON re-serialization of {@link dag_jack_example_document} — see {@link jack_example_json}.
pub fn dag_jack_example_json() -> String {
    semio_framework_pack_json::to_json_string(&dag_jack_example_document())
}
//#endregion 🔖️Examples

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;


#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::operations::*;
use crate::schema::mutations::WriterMutation;
#[cfg(test)]
use crate::schema::mutations::{ChangeLanguage, ChangeUri, EditText, RenameWriter};
use crate::WriterDiff;
use crate::WriterSnapshot;
use protocol::Mutation;

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_writer_snapshot_json(text: &str) -> Result<WriterSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_writer_snapshot_json(snapshot: &WriterSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod snapshot_codec {
use crate::standards::v1::subsets::any::schema::*;
use crate::{document_child_handle_with_text, WriterDocumentChild, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_s_artifact_trinity_jack::lexer::{lex_spanned, SpannedToken, Token};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub(crate) fn byte_span_to_text_span(text: &str, start: usize, end: usize) -> semio_framework_diagnostic::TextSpan {
    let safe_end = end.min(text.len());
    let safe_start = start.min(safe_end);
    let prefix = &text[..safe_start];
    let line = prefix.chars().filter(|&c| c == '\n').count() as u32 + 1;
    let column = prefix.rfind('\n').map(|i| safe_start - i).unwrap_or(safe_start) as u32;
    let length = (safe_end - safe_start) as u32;
    semio_framework_diagnostic::TextSpan::with_length(line, column, length.max(1))
}

pub(crate) fn token_class_from_name(name: &str) -> semio_framework_dsl::TokenClass {
    match name {
        "keyword" => semio_framework_dsl::TokenClass::Keyword,
        "string" => semio_framework_dsl::TokenClass::String,
        "number" => semio_framework_dsl::TokenClass::Number,
        "operator" => semio_framework_dsl::TokenClass::Operator,
        "comment" => semio_framework_dsl::TokenClass::Comment,
        "error" => semio_framework_dsl::TokenClass::Error,
        _ => semio_framework_dsl::TokenClass::Ident,
    }
}

/// 🎼️ The `jack` query-language `dsl::DslIdiom`, used directly by writer's jack completion surface.
pub(crate) struct JackWriterIdiom;

impl semio_framework_dsl::DslIdiom for JackWriterIdiom {
    const LANG: &'static str = "jack";
    type Ast = String;

    fn parse(text: &str) -> Result<Self::Ast, semio_framework_diagnostic::TextError> {
        semio_s_artifact_trinity_jack::core::format(text).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print(ast: &Self::Ast) -> String {
        ast.clone()
    }

    fn classify(text: &str) -> Vec<(semio_framework_dsl::TokenClass, semio_framework_diagnostic::TextSpan)> {
        semio_s_artifact_trinity_jack::core::semantic_tokens(text).into_iter().map(|t| (token_class_from_name(&t.class), byte_span_to_text_span(text, t.start, t.end))).collect()
    }

    fn complete(text: &str, offset: usize) -> Vec<semio_framework_dsl::CompletionItem> {
        let graph = semio_s_artifact_trinity_jack::core::example_graph();
        semio_s_artifact_trinity_jack::core::complete(&graph, text, offset).into_iter().map(|item| semio_framework_dsl::CompletionItem { label: item.label, detail: item.detail }).collect()
    }
}

/// 🎼️ The `wire` protocol-text `dsl::DslIdiom` — see [`JackWriterIdiom`]'s doc comment for why it lives here.
pub(crate) struct WireWriterIdiom;

impl semio_framework_dsl::DslIdiom for WireWriterIdiom {
    const LANG: &'static str = "wire";
    type Ast = String;

    fn parse(text: &str) -> Result<Self::Ast, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::parse_wire_text(text.trim())?;
        Ok(text.to_string())
    }

    fn print(ast: &Self::Ast) -> String {
        ast.clone()
    }

    fn classify(text: &str) -> Vec<(semio_framework_dsl::TokenClass, semio_framework_diagnostic::TextSpan)> {
        let limits = semio_framework_diagnostic::Limits::default();
        let Ok(tokens) = semio_framework_dsl::lex(text, &limits, false) else {
            return Vec::new();
        };
        tokens
            .into_iter()
            .filter(|t| !t.kind.is_trivia() && t.kind != semio_framework_dsl::TokenKind::Eof)
            .map(|t| {
                let class = match t.kind {
                    semio_framework_dsl::TokenKind::Arrow | semio_framework_dsl::TokenKind::DashArrow | semio_framework_dsl::TokenKind::EdgeArrow | semio_framework_dsl::TokenKind::BackArrow => semio_framework_dsl::TokenClass::Operator,
                    semio_framework_dsl::TokenKind::Float | semio_framework_dsl::TokenKind::Int => semio_framework_dsl::TokenClass::Number,
                    semio_framework_dsl::TokenKind::Text => semio_framework_dsl::TokenClass::String,
                    semio_framework_dsl::TokenKind::Ident => semio_framework_dsl::TokenClass::Ident,
                    _ => semio_framework_dsl::TokenClass::Punctuation,
                };
                let start = t.byte_range.0 as usize;
                let end = t.byte_range.1 as usize;
                (class, byte_span_to_text_span(text, start, end))
            })
            .collect()
    }
}

pub fn language_completions_json(text: &str, language_id: &str, cursor: usize) -> Option<String> {
    if let Some(spec) = semio_framework_dsl::language(language_id) {
        let session = dsl::lsp::LanguageSession::open(spec, text.to_string());
        let items: Vec<Value> = session.completions_at(cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
        return serde_json::to_string(&items).ok();
    }
    if let Some(hooks) = semio_framework_dsl::idiom(language_id) {
        let items: Vec<Value> = (hooks.complete)(text, cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
        return serde_json::to_string(&items).ok();
    }
    None
}

pub fn jack_completions_json(text: &str, cursor: usize) -> Option<String> {
    let items: Vec<Value> = <JackWriterIdiom as semio_framework_dsl::DslIdiom>::complete(text, cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
    serde_json::to_string(&items).ok()
}

/// 🎼️ `wire` counterpart of [`jack_completions_json`] — see [`WireWriterIdiom`]'s doc comment.
pub fn wire_completions_json(text: &str, cursor: usize) -> Option<String> {
    let items: Vec<Value> = <WireWriterIdiom as semio_framework_dsl::DslIdiom>::complete(text, cursor).into_iter().map(|item| json!({ "label": item.label, "detail": item.detail })).collect();
    serde_json::to_string(&items).ok()
}
}
pub use snapshot_codec::*;
