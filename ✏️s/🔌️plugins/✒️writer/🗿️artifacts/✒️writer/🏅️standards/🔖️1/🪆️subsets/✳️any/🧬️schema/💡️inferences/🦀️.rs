//! 💡️ Writer inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::WriterSnapshot;
use framework_schema::ArtifactSchema;
use semio_s_artifact_trinity_jack::core::{example_graph, lint};
use serde::{Deserialize, Serialize};
use serde_json::json;
//#region 🔖️Inference
/// 💡️ Everything inferable from a writer snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir) — writer is a
/// plain-text document with no structured fields, so its "outline" is derived straight from the
/// `text` field: markdown-style `#` headings plus real word/line counts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer.inference")]
pub struct WriterInference {
    #[derived]
    pub outline: WriterOutline,
}

impl protocol::Inference<WriterSnapshot> for WriterInference {
    fn infer(snapshot: &WriterSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: WriterOutline::compute(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `WriterSnapshot::default()`'s `text` field ever stops being empty (same trick the sequence
/// plugin's own inference facet uses for its non-empty default snapshot).
impl Default for WriterInference {
    fn default() -> Self {
        let snapshot = &WriterSnapshot::default();

        Self { outline: WriterOutline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<WriterSnapshot> for WriterInference {
    fn inference_schema_id() -> &'static str {
        "s.writer.writer.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.writer.writer.inference.outline", reads: &["text"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.writer.writer.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `writer_artifact_schema_descriptor`'s registration.
pub fn writer_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.writer.writer.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️LanguageInferences
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
//#endregion 🔖️LanguageInferences

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::outline::WriterOutline;
//#endregion 🔁️Re-exports
