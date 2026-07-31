//! 🗺️ GIS 2D app — document entities (constitutional: general).

use protocol::{Identified, Patchable};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub const GIS_MAP_SCHEMA: &str = "gis.map";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🗺️ One id-keyed spatial feature (a position pin, route polyline, or region ring) carried as its full
/// opaque descriptor payload — id-keyed so two authors editing different features converge granularly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeature {
    #[dsl(positional)]
    pub id: String,
    pub data: serde_json::Value,
}

impl Identified<String> for MapFeature {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Whole-payload replacement patch (features are opaque JSON); inverts to the prior payload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeaturePatch {
    pub data: Option<serde_json::Value>,
}

impl Patchable<MapFeaturePatch> for MapFeature {
    fn apply_patch(&mut self, patch: &MapFeaturePatch) {
        if let Some(data) = &patch.data {
            self.data = data.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<MapFeaturePatch> {
        (self.data != other.data).then(|| MapFeaturePatch { data: Some(other.data.clone()) })
    }
}

/// 🗺️ The editable map document: three id-keyed feature collections. All view/config state (camera,
/// render mode, vector style, LOD, selection, layer visibility, stroke weights) is plugin runtime, not
/// document state, so panning and styling never enter undo history.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "gismap", layout = "lines")]
pub struct GisMapDocument {
    #[serde(default)]
    #[dsl(table)]
    pub positions: Vec<MapFeature>,
    #[serde(default)]
    #[dsl(table)]
    pub routes: Vec<MapFeature>,
    #[serde(default)]
    #[dsl(table)]
    pub regions: Vec<MapFeature>,
}
//#endregion 🔖️Types
