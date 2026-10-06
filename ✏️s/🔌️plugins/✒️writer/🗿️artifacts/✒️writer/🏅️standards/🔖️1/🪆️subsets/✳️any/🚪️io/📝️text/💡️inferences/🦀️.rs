//! 📖️ Writer inference — the normative handcrafted text grammar for this facet. Inference
//! values are never authored via DSL text (they are always computed from a snapshot, never a
//! source of truth), so — unlike `📸️snapshot/📝️text`'s live `parse_dsl`/`print_dsl` pair — this
//! leaf declares the wire grammar only, matching the generic header/payload scaffold shape every
//! other representation leaf in this tree already uses for its own facet.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

#[allow(unused_imports)]
mod inferences_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::inferences::*;
use crate::WriterSnapshot;
use framework_schema::ArtifactSchema;
use semio_s_artifact_trinity_jack::core::{example_graph, lint};
use serde::{Deserialize, Serialize};
use serde_json::json;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1::subsets::any::schema::inferences::outline::WriterOutline;

/// 📡️ Semantic token payload for the text editor scene (LSP `data` array or grammar tokens) — derived
/// straight from a `WriterSnapshot` (its `language_id`/`text` fields), so it lives here beside
/// `WriterInference` rather than in `🧬️schema`'s text-only helpers.
pub fn language_tokens_json(document: &WriterSnapshot) -> Option<String> {
    let text = crate::writer_text(document);
    if let Some(spec) = semio_framework_dsl::language(&document.language_id) {
        let session = dsl::lsp::LanguageSession::open(spec, text.clone());
        return Some(semio_framework_pack_json::to_json_string(&session.semantic_tokens_lsp()));
    }
    if semio_framework_dsl::idiom(&document.language_id).is_some() {
        let tokens = crate::schema::tokenize_language(&text, &document.language_id);
        return serde_json::to_string(&tokens).ok();
    }
    None
}

pub fn language_diagnostics_json(document: &WriterSnapshot, lint_signal: u32) -> Option<String> {
    let text = crate::writer_text(document);
    if document.language_id == "jack" {
        let graph = example_graph();
        let diagnostics: Vec<semio_framework_pack_json::Value> = lint(&graph, &text).into_iter().map(|diag| semio_framework_pack_json::json!({ "start": diag.start, "end": diag.end, "severity": diag.severity, "message": diag.message })).collect();
        return Some(semio_framework_pack_json::to_json_string(&diagnostics));
    }
    if let Some(hooks) = semio_framework_dsl::idiom(&document.language_id) {
        if let Err(err) = (hooks.canonicalize)(&text) {
            let end = text.len().max(1);
            return serde_json::to_string(&[json!({ "start": 0, "end": end, "severity": "error", "message": err.message })]).ok();
        }
    } else if let Some(spec) = semio_framework_dsl::language(&document.language_id) {
        let session = dsl::lsp::LanguageSession::open(spec, text.clone());
        if let Err(err) = session.canonicalize() {
            let end = text.len().max(1);
            return serde_json::to_string(&[json!({ "start": 0, "end": end, "severity": "error", "message": err.message })]).ok();
        }
    }
    if lint_signal > 0 {
        return Some(json!([{ "start": 0, "end": text.len().max(1), "severity": "info", "message": format!("Lint pass #{lint_signal}") }]).to_string());
    }
    None
}
}
pub use inferences_codec::*;
