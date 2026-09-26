//! 📥️ EN 1996 inputs window — structured subject editor.

use crate::En1996Snapshot;
use semio_framework_plugin::{LocalizedLabel, TreeWindows, WindowKindDefinition};

pub const WINDOW_INPUTS: &str = "norm-en1996-inputs";
pub const BODY_INPUTS: &str = "norm.en1996.play.inputs";

pub fn definition() -> WindowKindDefinition {
    crate::app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}

pub fn render(document: &En1996Snapshot, locale: semio_framework_plugin::Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::app_surface::render_document_editor(document, locale, controller_id, Some(crate::field_meta::en1996_field_meta), windows)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
