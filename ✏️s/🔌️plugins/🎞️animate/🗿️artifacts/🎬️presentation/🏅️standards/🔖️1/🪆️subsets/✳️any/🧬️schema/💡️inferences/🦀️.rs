//! 💡️ Presentation inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::presentation::PresentationSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_presentation_topology, PresentationTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a presentation snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — the tile
/// filmstrip has no dependency edges of its own, so "topology" here is the honest degenerate case:
/// a linear chain in persisted tile order (`topoOrder` == tile ids in order, `depth` == each tile's
/// index, always `cycleFree`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.presentation.inference")]
pub struct PresentationInference {
    #[derived]
    pub topology: PresentationTopology,
}

impl protocol::Inference<PresentationSnapshot> for PresentationInference {
    fn infer(snapshot: &PresentationSnapshot) -> Self {
        Self { topology: compute_presentation_topology(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) so this stays correct even if `PresentationSnapshot`'s
/// own `Default` ever stops being the empty tile list — see the sibling artifacts' inference
/// families for the same trick where the default snapshot is NOT the zero value.
impl Default for PresentationInference {
    fn default() -> Self {
        <Self as protocol::Inference<PresentationSnapshot>>::infer(&PresentationSnapshot::default())
    }
}

impl protocol::InferenceSpec<PresentationSnapshot> for PresentationInference {
    fn inference_schema_id() -> &'static str {
        "s.animate.presentation.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.animate.presentation.inference.topology", reads: &["tiles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🎯️ Local zero-sized marker (not `schema::Construction`/`SnapshotBuilder<S, M>` — that is a
/// foreign generic struct, so `impl ArtifactInferrer for SnapshotBuilder<PresentationSnapshot,
/// PresentationMutation>` is an orphan-rule violation, E0117; confirmed by `🎬️sequence`'s identical
/// pass, ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM `📓️w4-sequence-report.md`
/// `## recipeGaps` #1). `ArtifactInferrer::infer` takes `&Self::Snapshot`, never `&self`, so this
/// type is a pure type-level anchor.
pub struct PresentationInferrer;

impl ArtifactInferrer for PresentationInferrer {
    type Snapshot = PresentationSnapshot;
    type Inference = PresentationInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.animate.presentation.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `presentation_artifact_schema_descriptor`'s registration.
pub fn presentation_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.animate.presentation.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::presentation::{FigureTileDraft, FigureTileFrame};
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn tile(id: &str, name: &str) -> FigureTileDraft {
        FigureTileDraft { id: id.into(), name: name.into(), crop: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }
    }

    fn sample_snapshot() -> PresentationSnapshot {
        let (source, _) = crate::artifacts::presentation::presentation_working_scene(&PresentationSnapshot::default());
        crate::artifacts::presentation::presentation_snapshot_with_tiles(&source, &[tile("tile-1", "First"), tile("tile-2", "Second"), tile("tile-3", "Third")])
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(PresentationInference::infer(&snapshot), PresentationInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(PresentationInference::infer(&PresentationSnapshot::default()), PresentationInference::default());
    }

    #[test]
    fn topology_orders_tiles_by_persisted_position() {
        let snapshot = sample_snapshot();
        let inferred = PresentationInference::infer(&snapshot);
        assert_eq!(inferred.topology.topo_order, vec!["tile-1".to_string(), "tile-2".to_string(), "tile-3".to_string()]);
        assert_eq!(inferred.topology.depth.get("tile-2"), Some(&1));
        assert!(inferred.topology.cycle_free);
        assert_eq!(inferred.topology.node_count, 3);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
