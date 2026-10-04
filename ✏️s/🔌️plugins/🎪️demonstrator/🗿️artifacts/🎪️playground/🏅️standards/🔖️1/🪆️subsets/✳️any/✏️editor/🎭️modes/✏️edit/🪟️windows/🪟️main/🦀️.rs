//! 🪟️ Playground editor — the `main` window: the document's one `schema` field as an editable text
//! buffer, built from the framework's `TextWindowKit` (contract §2.6).

use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_framework_plugin::app::{TextEditView, TextWindowKit, WindowKit};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::playground::create_playground_editor`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Schema", "Schema"), ..TextWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Pure `PlaygroundSnapshot -> UiNode` read: the document's one `schema` metadata string as the kit's explicit draft — edited
/// locally, one framework-catalog `textEdit` on Apply; it and the surface's own `changeSchema` manifest action both
/// dispatch through `PlaygroundEditor::handle`'s one `PlaygroundCommand::ChangeSchema` row.
pub fn render(document: &PlaygroundSnapshot, locale: Locale, publication_revision: semio_framework_ui_contract::UiPublicationRevision) -> UiAssemblyResult<BuiltNode> {
    TextWindowKit::render_editable(&TextEditView { surface_id: WINDOW_KIND_ID.into(), text: document.schema.clone(), language: Some("playground".into()), revision: None, publication_revision }, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
