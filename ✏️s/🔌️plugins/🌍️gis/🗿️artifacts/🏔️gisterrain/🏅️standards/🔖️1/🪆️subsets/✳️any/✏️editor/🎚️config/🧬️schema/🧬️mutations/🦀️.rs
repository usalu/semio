//! 🧬️ Transparent GIS 3D configuration mutation roster.
use super::{Gis3dConfig, Gis3dConfigDiff};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🧬️Leaves
#[path = "🎥️set-camera/🦀️.rs"] mod set_camera;
pub use set_camera::SetCamera;
//#endregion 🧬️Leaves
//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::Mutations, dsl::DslOps, ToValue, FromValue)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = Gis3dConfig, diff = Gis3dConfigDiff, schema = "gis.gis3dcfg")]
pub enum Gis3dConfigMutation { SetCamera(SetCamera) }
//#endregion 🧬️Aggregate
