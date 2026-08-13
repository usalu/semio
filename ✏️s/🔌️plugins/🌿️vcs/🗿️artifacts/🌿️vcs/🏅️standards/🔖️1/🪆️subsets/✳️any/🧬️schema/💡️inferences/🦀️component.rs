//! 💡️ VCS inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📊summary/`).
//!
//! The VCS snapshot is `schema`/`title`/`counter`/`notes`/`status`/`tags` — no graph, no document
//! tree, no geometry. The only honest whole-snapshot derivation is a scalar digest of the two
//! free-form fields (`tags`, `notes`), so this uses the plain `protocol::Inference<P>` shape (no
//! `InferredField`/caching machinery — nothing here is per-entity or incremental).

use crate::artifacts::vcs::VcsSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::summary::compute_vcs_summary;

//#region 🔖️Inference
/// 💡️ Everything inferable from a VCS snapshot. One field per named inference under
/// `💡️inferences/` (currently: `summary`, backed by the `📊summary/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.vcs.vcs.inference")]
pub struct VcsInference {
    #[derived]
    pub summary: VcsSummary,
}

impl protocol::Inference<VcsSnapshot> for VcsInference {
    fn infer(snapshot: &VcsSnapshot) -> Self {
        Self { summary: compute_vcs_summary(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `VcsSnapshot::default()`'s `tags`/`notes` ever stop being empty. Same "match `infer` of the real
/// default, don't derive structurally" trick `AddInference` uses in `📡️spr/🎮️command/🦀️component.rs`.
impl Default for VcsInference {
    fn default() -> Self {
        <Self as protocol::Inference<VcsSnapshot>>::infer(&VcsSnapshot::default())
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

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::vcs::standards::v1::subsets::any::schema::VcsBuilder {
    type Snapshot = VcsSnapshot;
    type Inference = VcsInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.vcs.vcs.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `vcs_artifact_schema_descriptor`'s registration.
pub fn vcs_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.vcs.vcs.inference",
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

pub use super::summary::VcsSummary;

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    fn tagged_snapshot() -> VcsSnapshot {
        VcsSnapshot { tags: vec!["alpha".into(), "beta".into()], notes: "demo notes here".into(), ..VcsSnapshot::default() }
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = tagged_snapshot();
        assert_eq!(VcsInference::infer(&snapshot), VcsInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(VcsInference::infer(&VcsSnapshot::default()), VcsInference::default());
    }

    #[test]
    fn summary_counts_tags_and_words() {
        let inferred = VcsInference::infer(&tagged_snapshot());
        assert_eq!(inferred.summary.tag_count, 2);
        assert_eq!(inferred.summary.notes_word_count, 3);
        assert!(inferred.summary.has_notes);
    }
}
//#endregion 🧪️Tests
