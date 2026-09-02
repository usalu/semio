//! 🧬️ Transparent GIS 3D configuration mutation roster.
use super::{Gis3dConfig, Gis3dConfigDiff};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🧬️Leaves
#[path = "🎥️set-camera/🦀️.rs"] mod set_camera;
#[path = "🗣️set-locale/🦀️.rs"] mod set_locale;
pub use set_camera::SetCamera;
pub use set_locale::SetLocale;
//#endregion 🧬️Leaves
//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::Mutations, dsl::DslOps, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields))]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = Gis3dConfig, diff = Gis3dConfigDiff, schema = "gis.gis3dcfg")]
pub enum Gis3dConfigMutation { SetCamera(SetCamera), SetLocale(SetLocale) }
//#endregion 🧬️Aggregate
