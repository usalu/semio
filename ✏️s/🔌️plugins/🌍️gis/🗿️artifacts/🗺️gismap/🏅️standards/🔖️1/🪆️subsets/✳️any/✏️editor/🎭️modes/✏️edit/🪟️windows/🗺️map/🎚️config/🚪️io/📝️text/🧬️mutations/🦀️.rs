//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::gis2d::modes::edit::windows::map::config::component::mutations::*;
use crate::editor::gis2d::modes::edit::windows::map::config::component::{MapWindowConfig, MapWindowConfigDiff};
use semio_framework_value_derive::{FromValue, ToValue};
use set_camera::SetCamera;
use set_layer_stroke_scale::SetLayerStrokeScale;
use set_layer_visibility::SetLayerVisibility;
use set_lod_mode::SetLodMode;
use set_render_mode::SetRenderMode;
use set_vector_style::SetVectorStyle;

/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `MapWindowConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn map_window_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<MapWindowConfig, MapWindowConfigMutation>(base_json, mutation_json, after_json)
}
}
pub use mutations_codec::*;
