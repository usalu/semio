//! ⛰️ GIS 3D app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🗄️ VCS-backed, undoable document for GIS 3D — deliberately minimal for the first pass: the only
/// editable/undoable property is vertical exaggeration (a genuinely useful terrain control).
pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";
//#endregion 🔖️Constants

//#region 🔖️Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "gisterrain", keyword = "gisterrain")]
pub struct Gis3dTerrainDocument {
    pub exaggeration: f64,
    /// 🔌️ `map:in`'s insertion point (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE
    /// Wave 2 port recipe): the last-imported `2d.map` descriptor JSON (`{positions,routes,regions}`,
    /// same shape `gis2d_engine::gis_map_descriptor_json` produces), rendered as an extra pin layer
    /// alongside the read-only fixture-text positions (see `gis3d_ui::instances_json`) — real, undoable
    /// document state, not view-only scratch, since importing a map overlay is a document edit.
    pub imported_features_json: String,
}
//#endregion 🔖️Types
