//! 💡️ Writer inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::writer::WriterSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use trinity::core::{example_graph, lint};

use super::outline::WriterOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a writer snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir) — writer is a
/// plain-text document with no structured fields, so its "outline" is derived straight from the
/// `text` field: markdown-style `#` headings plus real word/line counts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer.inference")]
pub struct WriterInference {
    #[derived]
    pub outline: WriterOutline,
}

impl protocol::Inference<WriterSnapshot> for WriterInference {
    async fn infer(snapshot: &WriterSnapshot) -> Self {
        Self { outline: WriterOutline::compute(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `WriterSnapshot::default()`'s `text` field ever stops being empty (same trick the sequence
/// plugin's own inference facet uses for its non-empty default snapshot).
impl Default for WriterInference {
    fn default() -> Self {
        <Self as protocol::Inference<WriterSnapshot>>::infer(&WriterSnapshot::default())
    }
}

impl protocol::InferenceSpec<WriterSnapshot> for WriterInference {
    async fn inference_schema_id() -> &'static str {
        "s.writer.writer.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.writer.writer.inference.outline", reads: &["text"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🪧️ Zero-sized marker anchor for `ArtifactInferrer::infer` (takes `&Self::Snapshot`, never
/// `&self` — a pure type-level anchor, no live callers by value). NOT `semio_framework_plugin::
/// app::SnapshotBuilder<WriterSnapshot, WriterMutation>`: that is a foreign, non-`#[fundamental]`
/// generic struct, so `impl ArtifactInferrer for SnapshotBuilder<Local, Local>` is an orphan-rule
/// violation (E0117) regardless of the type parameters being local — see `📓️w4-sequence-report.md`
/// `## recipeGaps` #1, the first agent to hit and document this exact trap.
pub struct WriterInferrer;
impl ArtifactInferrer for WriterInferrer {
    type Snapshot = WriterSnapshot;
    type Inference = WriterInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.writer.writer.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `writer_artifact_schema_descriptor`'s registration.
pub async fn writer_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.writer.writer.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️LanguageInferences
/// 📡️ Semantic token payload for the text editor scene (LSP `data` array or grammar tokens) — derived
/// straight from a `WriterSnapshot` (its `language_id`/`text` fields), so it lives here beside
/// `WriterInference` rather than in `🧬️schema`'s text-only helpers.
pub async fn language_tokens_json(document: &WriterSnapshot) -> Option<String> {
    let text = crate::artifacts::writer::writer_text(document);
    eprintln!("[DEBUG] writer.schema.inferences language_tokens_json language_id={} text_len={}", document.language_id, text.len());
    if let Some(spec) = dsl::language(&document.language_id) {
        let session = dsl::lsp::LanguageSession::open(spec, text.clone());
        return serde_json::to_string(&session.semantic_tokens_lsp()).ok();
    }
    if dsl::idiom(&document.language_id).is_some() {
        let tokens = crate::artifacts::writer::schema::tokenize_language(&text, &document.language_id);
        return serde_json::to_string(&tokens).ok();
    }
    None
}

pub async fn language_diagnostics_json(document: &WriterSnapshot, lint_signal: u32) -> Option<String> {
    let text = crate::artifacts::writer::writer_text(document);
    if document.language_id == "jack" {
        let graph = example_graph();
        let diagnostics: Vec<Value> = lint(&graph, &text).into_iter().map(|diag| json!({ "start": diag.start, "end": diag.end, "severity": diag.severity, "message": diag.message })).collect();
        return serde_json::to_string(&diagnostics).ok();
    }
    if let Some(hooks) = dsl::idiom(&document.language_id) {
        if let Err(err) = (hooks.canonicalize)(&text) {
            let end = text.len().max(1);
            return serde_json::to_string(&[json!({ "start": 0, "end": end, "severity": "error", "message": err.message })]).ok();
        }
    } else if let Some(spec) = dsl::language(&document.language_id) {
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
mod tests {
    use super::*;
    use protocol::Inference;

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = WriterSnapshot::default();
        assert_eq!(WriterInference::infer(&snapshot), WriterInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(WriterInference::infer(&WriterSnapshot::default()), WriterInference::default());
    }
}
//#endregion 🧪️Tests
