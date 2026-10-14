//! 📥️ EN 1990 play app — the inputs window: the raw compliance document, rendered as JSON.

use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::WindowKindDefinition;

//#region 🔖️Constants
pub const WINDOW_INPUTS: &str = "norm-en1990-inputs";
pub const BODY_INPUTS: &str = "norm.en1990.play.inputs";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::en1990::create_en1990_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &En1990Snapshot, locale: semio_framework_ui_locale::Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_s_plugin_norm_app_surface::render_document_editor(document, locale, controller_id, Some(crate::field_meta::en1990_field_meta), windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
