//! 💡️ GIS terrain inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::artifacts::gisterrain::GisTerrainSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::bounds::{imported_lon_lat_positions, lon_lat_bounds, GisTerrainBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a gisterrain snapshot. Today: the geographic bounding box and
/// position count of the `map:in` overlay decoded from `imported_features_json` (see
/// `📦bounds/🦀️component.rs`). A simple whole-snapshot scalar — no `InferredField` caching, the
/// overlay is small and re-decoding is O(positions).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gisterrain.inference")]
pub struct GisTerrainInference {
    #[state(inferred)]
    pub position_count: usize,
    #[state(inferred)]
    pub bounds: Option<GisTerrainBounds>,
}

impl protocol::Inference<GisTerrainSnapshot> for GisTerrainInference {
    fn infer(snapshot: &GisTerrainSnapshot) -> Self {
        let positions = imported_lon_lat_positions(snapshot);
        Self { position_count: positions.len(), bounds: lon_lat_bounds(&positions) }
    }
}

impl protocol::InferenceSpec<GisTerrainSnapshot> for GisTerrainInference {
    fn inference_schema_id() -> &'static str {
        "s.gis.gisterrain.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.gis.gisterrain.inference.positionCount", reads: &["importedFeaturesJson"] },
            protocol::InferenceFieldSpec { id: "s.gis.gisterrain.inference.bounds", reads: &["importedFeaturesJson"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::gisterrain::standards::v1::subsets::any::schema::GisterrainBuilder {
    type Snapshot = GisTerrainSnapshot;
    type Inference = GisTerrainInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.gis.gisterrain.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `gisterrain_artifact_schema_descriptor`'s registration.
pub fn gisterrain_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.gis.gisterrain.inference",
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

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = GisTerrainSnapshot {
            exaggeration: 1.5,
            imported_features_json: serde_json::json!({ "positions": [{ "id": "p1", "lon": 5.58, "lat": 50.60 }] }).to_string(),
        };
        assert_eq!(GisTerrainInference::infer(&snapshot), GisTerrainInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(GisTerrainInference::infer(&GisTerrainSnapshot::default()), GisTerrainInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
