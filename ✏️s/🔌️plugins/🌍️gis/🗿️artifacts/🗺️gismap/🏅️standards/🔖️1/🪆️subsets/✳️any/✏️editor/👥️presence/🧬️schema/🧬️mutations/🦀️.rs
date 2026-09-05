//! 🧬️ Transparent GIS 2D presence mutation roster.

use super::{Gis2dPresence, Gis2dPresenceDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Leaves
#[path = "🎥️set-camera/🦀️.rs"] mod set_camera;
pub use set_camera::SetCamera;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::Mutations, dsl::DslOps, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields))]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = Gis2dPresence, diff = Gis2dPresenceDiff, schema = "gis.gis2dpresence")]
pub enum Gis2dPresenceMutation {
    SetCamera(SetCamera),
}
//#endregion 🧬️Aggregate
