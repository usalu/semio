//! 🪟️ Playground viewer — the `main` window: the document's one `schema` field as a read-only text
//! buffer, built from the framework's `TextWindowKit` (contract §2.6). Reads `PlaygroundSnapshot`
//! directly — no other module's render logic is needed for a single scalar field, so this file never
//! reaches into the sibling authoring surface for it.

use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
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
mod tests {
    use super::*;
    use semio_framework_plugin::Component;

    #[test]
    fn definition_declares_a_read_only_text_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
        assert!(def.actions.is_empty(), "a viewer window kind declares no mutation-shaped actions");
    }

    #[test]
    fn render_carries_the_schema_field_as_read_only_text() {
        let document = PlaygroundSnapshot { schema: "playground.custom".into() };
        let node = render(&document).expect("render");
        let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
        let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
        assert_eq!(scene.buffer, "playground.custom");
        assert_eq!(scene.language.as_deref(), Some("playground"));
        assert_eq!(scene.settings_json.as_deref(), Some("{\"readOnly\":true}"));
    }
}
//#endregion 🧪️Tests
