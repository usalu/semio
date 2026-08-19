//! 💡️ Sequence inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::sequence::SequenceSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_sequence_topology, SequenceTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a sequence snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — sequence is a
/// genuine step DAG (`steps` + `edges`), so `topology` here is a real Kahn's-algorithm topological
/// sort, not a degenerate stand-in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence.inference")]
pub struct SequenceInference {
    #[derived]
    pub topology: SequenceTopology,
}

impl protocol::Inference<SequenceSnapshot> for SequenceInference {
    async fn infer(snapshot: &SequenceSnapshot) -> Self {
        Self { topology: compute_sequence_topology(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SequenceSnapshot::default()` is the canonical
/// two-step example document (`default_snapshot()`), NOT the empty snapshot, so a naive derived
/// `Default` (all-zero fields) would disagree with `infer(&SequenceSnapshot::default())`. Computing
/// it via `infer` instead keeps the law correct regardless of what the default snapshot contains.
impl Default for SequenceInference {
    fn default() -> Self {
        <Self as protocol::Inference<SequenceSnapshot>>::infer(&SequenceSnapshot::default())
    }
}

impl protocol::InferenceSpec<SequenceSnapshot> for SequenceInference {
    async fn inference_schema_id() -> &'static str {
        "s.sequence.sequence.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.sequence.sequence.inference.topology", reads: &["content"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🌳️ Retargeted (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM) — the old
/// `derive_artifact_facets!`-generated `SequenceBuilderFacets` this impl targeted is deleted along
/// with the rest of the hand-rolled `ArtifactComposition`/`ArtifactAnalyzer` cluster (design.md §5
/// step 3). The recipe's suggested replacement (`semio_framework_plugin::app::SnapshotBuilder<S,
/// M>`) does NOT work here: `SnapshotBuilder` is a foreign (non-`#[fundamental]`) generic struct,
/// so `impl ArtifactInferrer for SnapshotBuilder<SequenceSnapshot, SequenceMutation>` is a genuine
/// orphan-rule violation (E0117) regardless of the type PARAMETERS being local — confirmed by
/// compiling it (see `📓️w4-sequence-report.md` `## recipeGaps`). `ArtifactInferrer::infer` takes
/// `&Self::Snapshot`, never `&self` — the impl target is a pure type-level anchor with zero live
/// callers repo-wide (grepped), so a trivial local zero-sized marker is the correct, minimal fix.
pub struct SequenceInferrer;
impl ArtifactInferrer for SequenceInferrer {
    type Snapshot = SequenceSnapshot;
    type Inference = SequenceInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.sequence.sequence.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `sequence_artifact_schema_descriptor`'s registration.
pub async fn sequence_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.sequence.sequence.inference",
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
    use crate::artifacts::sequence::{SequenceEdge, SequenceFixture, SequenceStep, StepParams};
    use protocol::Inference;

    //#region 🧸️Fixtures
    async fn step(id: &str) -> SequenceStep {
        SequenceStep { id: id.into(), kind: "state.set".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false }
    }

    async fn sample_snapshot() -> SequenceSnapshot {
        SequenceSnapshot::from_fixture(SequenceFixture {
            schema: crate::artifacts::sequence::SEQUENCE_DOCUMENT_SCHEMA.into(),
            steps: vec![step("a"), step("b")],
            edges: vec![SequenceEdge { id: "e1".into(), from: "a".into(), to: "b".into() }],
        })
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(SequenceInference::infer(&snapshot), SequenceInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SequenceInference::infer(&SequenceSnapshot::default()), SequenceInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
