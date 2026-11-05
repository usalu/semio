//! 📥️ VDI 3805 play app — the inputs window: the typed unit-aware inspector and entity outliner.

use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::TreeWindows;
use semio_plugin::WindowKindDefinition;

//#region 🔖️Constants
pub const WINDOW_INPUTS: &str = "norm-vdi3805-inputs";
pub const BODY_INPUTS: &str = "norm.vdi3805.play.inputs";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::vdi3805::create_vdi3805_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Vdi3805Snapshot, locale: semio_ui_locale::Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    semio_s_plugin_norm_app_surface::render_inspector_for::<crate::Vdi3805Family>(document, locale, controller_id, Some(crate::field_meta::vdi3805_field_meta), windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
