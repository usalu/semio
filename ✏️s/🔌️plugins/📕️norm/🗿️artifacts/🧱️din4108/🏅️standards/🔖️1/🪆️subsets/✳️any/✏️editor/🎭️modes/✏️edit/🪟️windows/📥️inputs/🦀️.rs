//! 📥️ DIN 4108 play app — the inputs window: structured document editor with field metadata.

use crate::Din4108Snapshot;
use semio_framework_plugin::{LocalizedLabel, TreeWindows, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_INPUTS: &str = "norm-din4108-inputs";
pub const BODY_INPUTS: &str = "norm.din4108.play.inputs";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::din4108::create_din4108_app`.
pub fn definition() -> WindowKindDefinition {
    crate::app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Din4108Snapshot, locale: semio_framework_plugin::Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_document_editor(document, locale, controller_id, Some(crate::field_meta::din4108_field_meta), windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
