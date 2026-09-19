//! 🧬️ Transparent GIS 3D configuration mutation roster.
use super::{GisTerrainWindowConfig, GisTerrainWindowConfigDiff};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🧬️Leaves
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
//#endregion 🧬️Leaves
//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::Mutations, dsl::DslOps, ToValue, FromValue)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = GisTerrainWindowConfig, diff = GisTerrainWindowConfigDiff, schema = "gis.gisterrainwindowcfg")]
pub enum GisTerrainWindowConfigMutation {
    SetCamera(SetCamera),
}
//#endregion 🧬️Aggregate
