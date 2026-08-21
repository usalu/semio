//! 💡️ GIS map inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::artifacts::gismap::GisMapSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::bounds::{all_lon_lat_pairs, lon_lat_bounds, GisMapBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a gismap snapshot. Today: per-collection feature counts and the
/// geographic bounding box across every `positions`/`routes`/`regions` feature (see
/// `📦bounds/🦀️component.rs`). A simple whole-snapshot scalar — no `InferredField` caching, the
/// feature collections here are small.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gismap.inference")]
pub struct GisMapInference {
    #[derived]
    pub position_count: usize,
    #[derived]
    pub route_count: usize,
    #[derived]
    pub region_count: usize,
    #[derived]
    pub bounds: Option<GisMapBounds>,
}

impl protocol::Inference<GisMapSnapshot> for GisMapInference {
    async fn infer(snapshot: &GisMapSnapshot) -> Self {
        Self { position_count: snapshot.positions.len(), route_count: snapshot.routes.len(), region_count: snapshot.regions.len(), bounds: lon_lat_bounds(&all_lon_lat_pairs(snapshot)) }
    }
}

impl protocol::InferenceSpec<GisMapSnapshot> for GisMapInference {
    async fn inference_schema_id() -> &'static str {
        "s.gis.gismap.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.positionCount", reads: &["positions"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.routeCount", reads: &["routes"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.regionCount", reads: &["regions"] },
            protocol::InferenceFieldSpec { id: "s.gis.gismap.inference.bounds", reads: &["positions", "routes", "regions"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::gismap::standards::v1::subsets::any::schema::GismapBuilder {
    type Snapshot = GisMapSnapshot;
    type Inference = GisMapInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.gis.gismap.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `gismap_artifact_schema_descriptor`'s registration.
pub async fn gismap_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.gis.gismap.inference",
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
    use crate::artifacts::gismap::MapFeature;
    use protocol::Inference;

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot =
            GisMapSnapshot { positions: vec![MapFeature { id: "p1".into(), data: dsl::to_dsl_value(&serde_json::json!({ "lon": 1.0, "lat": 2.0 })).unwrap_or(dsl::DslValue::Null) }], routes: Vec::new(), regions: Vec::new(), ..Default::default() };
        assert_eq!(GisMapInference::infer(&snapshot), GisMapInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(GisMapInference::infer(&GisMapSnapshot::default()), GisMapInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
