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
    async fn infer(snapshot: &VcsSnapshot) -> Self {
        Self { summary: compute_vcs_summary(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `VcsSnapshot::default()`'s `tags`/`notes` ever stop being empty. Same "match `infer` of the real
/// default, don't derive structurally" trick `AddInference` uses in `📡️spr/🎮️command/🦀️component.rs`.
impl Default for VcsInference {
    async fn default() -> Self {
        <Self as protocol::Inference<VcsSnapshot>>::infer(&VcsSnapshot::default())
    }
}

impl protocol::InferenceSpec<VcsSnapshot> for VcsInference {
    async fn inference_schema_id() -> &'static str {
        "s.vcs.vcs.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.vcs.vcs.inference.summary", reads: &["tags", "notes"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🌳️ Retargeted (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM) — the old
/// `derive_artifact_facets!`-generated `VcsBuilder` this impl targeted is deleted along with the
/// rest of the hand-rolled `ArtifactComposition`/`ArtifactAnalyzer` cluster (design.md §5 step 3).
/// The recipe's suggested replacement (`semio_framework_plugin::app::SnapshotBuilder<S, M>`) does
/// NOT work here: `SnapshotBuilder` is a foreign (non-`#[fundamental]`) generic struct, so `impl
/// ArtifactInferrer for SnapshotBuilder<VcsSnapshot, VcsDemoMutation>` is a genuine orphan-rule
/// violation (E0117) regardless of the type PARAMETERS being local (see `📓️w4-sequence-report.md`
/// `## recipeGaps` — confirmed by the sequence pilot actually compiling it). `ArtifactInferrer::infer`
/// takes `&Self::Snapshot`, never `&self` — the impl target is a pure type-level anchor with zero
/// live callers repo-wide (grepped), so a trivial local zero-sized marker is the correct, minimal fix.
pub struct VcsInferrer;
impl ArtifactInferrer for VcsInferrer {
    type Snapshot = VcsSnapshot;
    type Inference = VcsInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.vcs.vcs.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `vcs_artifact_schema_descriptor`'s registration.
pub async fn vcs_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
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

    async fn tagged_snapshot() -> VcsSnapshot {
        VcsSnapshot { tags: vec!["alpha".into(), "beta".into()], notes: "demo notes here".into(), ..VcsSnapshot::default() }
    }

    #[test]
    async fn inference_determinism_law() {
        let snapshot = tagged_snapshot();
        assert_eq!(VcsInference::infer(&snapshot), VcsInference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(VcsInference::infer(&VcsSnapshot::default()), VcsInference::default());
    }

    #[test]
    async fn summary_counts_tags_and_words() {
        let inferred = VcsInference::infer(&tagged_snapshot());
        assert_eq!(inferred.summary.tag_count, 2);
        assert_eq!(inferred.summary.notes_word_count, 3);
        assert!(inferred.summary.has_notes);
    }
}
//#endregion 🧪️Tests
