//! 📥️ EN 1997 inputs window — structured subject editor with localized field metadata.

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_checks::lookup_field_meta;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::WindowKindDefinition;

pub const WINDOW_INPUTS: &str = "norm-en1997-inputs";
pub const BODY_INPUTS: &str = "norm.en1997.play.inputs";

pub fn definition() -> WindowKindDefinition {
    semio_s_plugin_norm_app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}

pub fn render(document: &En1997Snapshot, locale: semio_framework_ui_locale::Locale, controller_id: &'static str, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_s_plugin_norm_app_surface::render_document_editor(document, locale, controller_id, Some(lookup_field_meta), windows)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
