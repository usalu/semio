//! 📥️ ISO 16757 play app — structured catalogue editor (B2 field-meta + lazy windowed tree).

use crate::Iso16757Snapshot;
use semio_framework_plugin::{LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_INPUTS: &str = "norm-iso16757-inputs";
pub const BODY_INPUTS: &str = "norm.iso16757.play.inputs";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::iso16757::create_iso16757_app`.
pub fn definition() -> WindowKindDefinition {
    crate::app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Iso16757Snapshot, locale: semio_framework_plugin::Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_document_editor(document, locale, controller_id, Some(crate::field_meta::iso16757_field_meta), windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
