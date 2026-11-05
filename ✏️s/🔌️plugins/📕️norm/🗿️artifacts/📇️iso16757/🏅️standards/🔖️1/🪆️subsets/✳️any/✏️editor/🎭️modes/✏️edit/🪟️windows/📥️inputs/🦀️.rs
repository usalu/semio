//! 📥️ ISO 16757 play app — structured catalogue editor (B2 field-meta + lazy windowed tree).

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::TreeWindows;
use semio_plugin::WindowKindDefinition;

//#region 🔖️Constants
pub const WINDOW_INPUTS: &str = "norm-iso16757-inputs";
pub const BODY_INPUTS: &str = "norm.iso16757.play.inputs";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::iso16757::create_iso16757_app`.
pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Iso16757Snapshot, locale: semio_ui_locale::Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    semio_s_plugin_norm_app_surface::render_inspector_for::<crate::Iso16757Family>(document, locale, controller_id, Some(crate::field_meta::iso16757_field_meta), windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
