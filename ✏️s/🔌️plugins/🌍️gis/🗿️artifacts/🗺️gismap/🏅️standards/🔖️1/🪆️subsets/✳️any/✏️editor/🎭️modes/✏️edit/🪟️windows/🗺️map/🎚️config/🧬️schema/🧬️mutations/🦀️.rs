//! 🧬️ Transparent direct configuration mutation roster.

use super::{MapWindowConfig, MapWindowConfigDiff};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Leaves
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
#[path = "📏️set-layer-stroke-scale/🦀️.rs"]
mod set_layer_stroke_scale;
#[path = "👁️set-layer-visibility/🦀️.rs"]
mod set_layer_visibility;
#[path = "🔽️set-lod-mode/🦀️.rs"]
mod set_lod_mode;
#[path = "🖼️set-render-mode/🦀️.rs"]
mod set_render_mode;
#[path = "🎨️set-vector-style/🦀️.rs"]
mod set_vector_style;
pub use set_camera::SetCamera;
pub use set_layer_stroke_scale::SetLayerStrokeScale;
pub use set_layer_visibility::SetLayerVisibility;
pub use set_lod_mode::SetLodMode;
pub use set_render_mode::SetRenderMode;
pub use set_vector_style::SetVectorStyle;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, dsl::Mutations, dsl::DslOps, ToValue, FromValue)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = MapWindowConfig, diff = MapWindowConfigDiff, schema = "gis.mapwindowcfg")]
pub enum MapWindowConfigMutation {
    SetLayerVisibility(SetLayerVisibility),
    SetCamera(SetCamera),
    SetRenderMode(SetRenderMode),
    SetVectorStyle(SetVectorStyle),
    SetLodMode(SetLodMode),
    SetLayerStrokeScale(SetLayerStrokeScale),
}
//#endregion 🧬️Aggregate

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `MapWindowConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn map_window_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<MapWindowConfig, MapWindowConfigMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
