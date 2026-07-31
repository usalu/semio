//! ⛰️ GIS 3D app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🗄️ VCS-backed, undoable document for GIS 3D — deliberately minimal for the first pass: the only
/// editable/undoable property is vertical exaggeration (a genuinely useful terrain control).
pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";
//#endregion 🔖️Constants

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "gisterrain", keyword = "gisterrain")]
pub struct Gis3dTerrainDocument {
    pub exaggeration: f64,
}
//#endregion 🔖️Types
