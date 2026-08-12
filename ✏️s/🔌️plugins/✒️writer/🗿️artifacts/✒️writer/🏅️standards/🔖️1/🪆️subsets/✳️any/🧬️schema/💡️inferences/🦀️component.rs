//! 💡️ Writer inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::writer::WriterSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

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
    #[state(inferred)]
    pub outline: WriterOutline,
}

impl protocol::Inference<WriterSnapshot> for WriterInference {
    fn infer(snapshot: &WriterSnapshot) -> Self {
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

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::writer::standards::v1::subsets::any::schema::WriterBuilder {
    type Snapshot = WriterSnapshot;
    type Inference = WriterInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.writer.writer.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `writer_artifact_schema_descriptor`'s registration.
pub fn writer_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
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

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[test]
    fn inference_determinism_law() {
        let snapshot = WriterSnapshot::default();
        assert_eq!(WriterInference::infer(&snapshot), WriterInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(WriterInference::infer(&WriterSnapshot::default()), WriterInference::default());
    }
}
//#endregion 🧪️Tests
