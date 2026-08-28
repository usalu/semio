//! 🧬️ Transparent GIS 2D presence mutation roster.

use super::{Gis2dPresence, Gis2dPresenceDiff};

//#region 🧬️Leaves
#[path = "🎥️set-camera/🦀️.rs"] mod set_camera;
pub use set_camera::SetCamera;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::Mutations, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = Gis2dPresence, diff = Gis2dPresenceDiff, schema = "gis.gis2dpresence")]
pub enum Gis2dPresenceMutation {
    SetCamera(SetCamera),
}
//#endregion 🧬️Aggregate
