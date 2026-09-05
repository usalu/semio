//! 🧬️ Transparent direct configuration mutation roster.

use super::{Gis2dConfig, Gis2dConfigDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Leaves
#[path = "👁️set-layer-visibility/🦀️.rs"] mod set_layer_visibility;
#[path = "🎥️set-camera/🦀️.rs"] mod set_camera;
#[path = "🖼️set-render-mode/🦀️.rs"] mod set_render_mode;
#[path = "🎨️set-vector-style/🦀️.rs"] mod set_vector_style;
#[path = "🔽️set-lod-mode/🦀️.rs"] mod set_lod_mode;
#[path = "📏️set-layer-stroke-scale/🦀️.rs"] mod set_layer_stroke_scale;
#[path = "🗣️set-locale/🦀️.rs"] mod set_locale;
pub use set_layer_visibility::SetLayerVisibility;
pub use set_camera::SetCamera;
pub use set_render_mode::SetRenderMode;
pub use set_vector_style::SetVectorStyle;
pub use set_lod_mode::SetLodMode;
pub use set_layer_stroke_scale::SetLayerStrokeScale;
pub use set_locale::SetLocale;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::Mutations, dsl::DslOps, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields))]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = Gis2dConfig, diff = Gis2dConfigDiff, schema = "gis.gis2dcfg")]
pub enum Gis2dConfigMutation {
    SetLayerVisibility(SetLayerVisibility),
    SetCamera(SetCamera),
    SetRenderMode(SetRenderMode),
    SetVectorStyle(SetVectorStyle),
    SetLodMode(SetLodMode),
    SetLayerStrokeScale(SetLayerStrokeScale),
    SetLocale(SetLocale),
}
//#endregion 🧬️Aggregate
