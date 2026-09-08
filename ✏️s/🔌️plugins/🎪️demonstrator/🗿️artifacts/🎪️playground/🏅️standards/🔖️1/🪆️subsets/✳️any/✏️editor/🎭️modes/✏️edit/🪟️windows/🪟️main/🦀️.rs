//! 🪟️ Playground editor — the `main` window: the document's one `schema` field as an editable text
//! buffer, built from the framework's `TextWindowKit` (contract §2.6).

use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, UiAssemblyResult, WindowKindDefinition};

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
/// ✏️ Pure `PlaygroundSnapshot -> UiNode` read: the document's one `schema` metadata string, editable
/// (`read_only: false`) — the framework-catalog `replace-text` action on this window kind, plus the
/// surface's own `changeSchema` manifest action, both dispatch through `PlaygroundEditor::handle`'s
/// one `PlaygroundCommand::ChangeSchema` row.
pub fn render(document: &PlaygroundSnapshot) -> UiAssemblyResult<BuiltNode> {
    TextWindowKit::render(&TextView { text: document.schema.clone(), language: Some("playground".into()), read_only: false })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
