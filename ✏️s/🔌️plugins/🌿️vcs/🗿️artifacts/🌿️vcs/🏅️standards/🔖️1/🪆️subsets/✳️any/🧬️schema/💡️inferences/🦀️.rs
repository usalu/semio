//! 💡️ VCS inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📊summary/`).
//!
//! The VCS snapshot is `schema`/`title`/`counter`/`notes`/`status`/`tags` — no graph, no document
//! tree, no geometry. The only honest whole-snapshot derivation is a scalar digest of the two
//! free-form fields (`tags`, `notes`), so this uses the plain `protocol::Inference<P>` shape (no
//! `InferredField`/caching machinery — nothing here is per-entity or incremental).

use crate::VcsSnapshot;
use framework_schema::ArtifactSchema;

use super::summary::compute_vcs_summary;

//#region 🔖️Inference
/// 💡️ Everything inferable from a VCS snapshot. One field per named inference under
/// `💡️inferences/` (currently: `summary`, backed by the `📊summary/` slug dir).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.vcs.vcs.inference")]
pub struct VcsInference {
    #[derived]
    pub summary: VcsSummary,
}

impl protocol::Inference<VcsSnapshot> for VcsInference {
    fn infer(snapshot: &VcsSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { summary: compute_vcs_summary(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `VcsSnapshot::default()`'s `tags`/`notes` ever stop being empty. Same "match `infer` of the real
/// default, don't derive structurally" trick `AddInference` uses in `📡️spr/🎮️command/🦀️.rs`.
impl Default for VcsInference {
    fn default() -> Self {
        let snapshot = &VcsSnapshot::default();

        Self { summary: compute_vcs_summary(snapshot) }
    }
}

impl protocol::InferenceSpec<VcsSnapshot> for VcsInference {
    fn inference_schema_id() -> &'static str {
        "s.vcs.vcs.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.vcs.vcs.inference.summary", reads: &["tags", "notes"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.vcs.vcs.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `vcs_artifact_schema_descriptor`'s registration.
pub fn vcs_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.vcs.vcs.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

pub use super::summary::VcsSummary;

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
