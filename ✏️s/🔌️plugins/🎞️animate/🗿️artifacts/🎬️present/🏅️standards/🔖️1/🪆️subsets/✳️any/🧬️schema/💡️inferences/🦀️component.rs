//! 💡️ Present inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::present::PresentSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_present_topology, PresentTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a present snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — the tile
/// filmstrip has no dependency edges of its own, so "topology" here is the honest degenerate case:
/// a linear chain in persisted tile order (`topoOrder` == tile ids in order, `depth` == each tile's
/// index, always `cycleFree`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present.inference")]
pub struct PresentInference {
    #[state(inferred)]
    pub topology: PresentTopology,
}

impl protocol::Inference<PresentSnapshot> for PresentInference {
    fn infer(snapshot: &PresentSnapshot) -> Self {
        Self { topology: compute_present_topology(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) so this stays correct even if `PresentSnapshot`'s
/// own `Default` ever stops being the empty tile list — see the sibling artifacts' inference
/// families for the same trick where the default snapshot is NOT the zero value.
impl Default for PresentInference {
    fn default() -> Self {
        <Self as protocol::Inference<PresentSnapshot>>::infer(&PresentSnapshot::default())
    }
}

impl protocol::InferenceSpec<PresentSnapshot> for PresentInference {
    fn inference_schema_id() -> &'static str {
        "s.animate.present.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.animate.present.inference.topology", reads: &["tiles"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::present::standards::v1::subsets::any::schema::PresentBuilderFacets {
    type Snapshot = PresentSnapshot;
    type Inference = PresentInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.animate.present.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `present_artifact_schema_descriptor`'s registration.
pub fn present_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.animate.present.inference",
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
    use crate::artifacts::present::{FigureTileDraft, FigureTileFrame};
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn tile(id: &str, name: &str) -> FigureTileDraft {
        FigureTileDraft { id: id.into(), name: name.into(), crop: FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }
    }

    fn sample_snapshot() -> PresentSnapshot {
        let mut snapshot = PresentSnapshot::default();
        snapshot.tiles = vec![tile("tile-1", "First"), tile("tile-2", "Second"), tile("tile-3", "Third")];
        snapshot
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(PresentInference::infer(&snapshot), PresentInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(PresentInference::infer(&PresentSnapshot::default()), PresentInference::default());
    }

    #[test]
    fn topology_orders_tiles_by_persisted_position() {
        let snapshot = sample_snapshot();
        let inferred = PresentInference::infer(&snapshot);
        assert_eq!(inferred.topology.topo_order, vec!["tile-1".to_string(), "tile-2".to_string(), "tile-3".to_string()]);
        assert_eq!(inferred.topology.depth.get("tile-2"), Some(&1));
        assert!(inferred.topology.cycle_free);
        assert_eq!(inferred.topology.node_count, 3);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
