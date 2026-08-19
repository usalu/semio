//! 🪟️ Playground editor — the `main` window: the document's one `schema` field as an editable text
//! buffer, built from the framework's `TextWindowKit` (contract §2.6).

use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use semio_framework_plugin::app::{TextView, TextWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TextWindowKit::KIND_ID;
pub const BODY_KEY: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::playground::create_playground_editor`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Schema", "Schema"), ..TextWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Pure `PlaygroundSnapshot -> UiNode` read: the document's one `schema` metadata string, editable
/// (`read_only: false`) — the framework-catalog `replace-text` action on this window kind, plus the
/// surface's own `changeSchema` manifest action, both dispatch through `PlaygroundEditor::handle`'s
/// one `PlaygroundCommand::ChangeSchema` row.
pub async fn render(document: &PlaygroundSnapshot) -> UiNode {
    TextWindowKit::render(&TextView { text: document.schema.clone(), language: Some("playground".into()), read_only: false })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_an_editable_text_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.iter().any(|action| action.id == "replace-text"), "editable text window must carry the replace-text catalog action");
    }

    #[semio_framework_async_macros::async_test]
    async fn render_carries_the_schema_field_as_editable_text() {
        let document = PlaygroundSnapshot { schema: "playground.custom".into() };
        let UiNode::ComponentScene(node) = render(&document) else { panic!("expected ComponentScene") };
        let scene = node.text_editor.expect("text_editor scene");
        assert_eq!(scene.buffer, "playground.custom");
        assert!(scene.settings_json.is_none(), "editable window must not stamp readOnly");
    }
}
//#endregion 🧪️Tests
