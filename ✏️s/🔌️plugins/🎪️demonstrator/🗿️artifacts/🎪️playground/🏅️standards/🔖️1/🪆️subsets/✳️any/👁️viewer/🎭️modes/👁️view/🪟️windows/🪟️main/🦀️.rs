//! 🪟️ Playground viewer — the `main` window: the document's one `schema` field as a read-only text
//! buffer, built from the framework's `TextWindowKit` (contract §2.6). Reads `PlaygroundSnapshot`
//! directly — no other module's render logic is needed for a single scalar field, so this file never
//! reaches into the sibling authoring surface for it.

use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, UiAssemblyResult, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::playground::create_playground_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Schema", "Schema"), ..TextWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `PlaygroundSnapshot -> UiNode` read, always `read_only: true` — a viewer never emits a
/// `replace-text` command.
pub fn render(document: &PlaygroundSnapshot) -> UiAssemblyResult<BuiltNode> {
    TextWindowKit::render(&TextView { text: document.schema.clone(), language: Some("playground".into()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
