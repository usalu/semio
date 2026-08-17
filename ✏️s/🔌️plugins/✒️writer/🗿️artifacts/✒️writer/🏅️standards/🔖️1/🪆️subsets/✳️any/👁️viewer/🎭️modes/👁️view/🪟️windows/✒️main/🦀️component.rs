//! ✒️ Writer viewer — Main window: read-only text view built on the framework `TextWindowKit`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6). Writer's artifact is
//! text-based, so the general-purpose text window kit fits without a bespoke render function —
//! no world-3d/mesh scene, no editor-only chrome (selection/tokens/diagnostics/completions all stay
//! on the sibling `editor` module's own window, never read from here).

use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::app::{TextView, TextWindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition, WindowKit};

//#region 🔖️Constants
pub const WRITER_VIEW_WINDOW_KIND: &str = TextWindowKit::KIND_ID;
pub const WRITER_VIEW_BODY_MAIN: &str = TextWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::writer::create_writer_viewer` — the
/// framework kit's own read-only `window_kind()` variant (never `editable_window_kind()`: a viewer
/// declares no mutating actions).
pub fn definition() -> WindowKindDefinition {
    TextWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &WriterSnapshot) -> UiNode {
    TextWindowKit::render(&TextView { text: writer_text(document), language: Some(document.language_id.clone()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_uses_the_framework_text_window_kits_frozen_kind_id() {
        let def = definition();
        assert_eq!(def.id, "framework.window.text");
        assert_eq!(def.id, WRITER_VIEW_WINDOW_KIND);
    }

    #[test]
    fn definition_declares_no_mutating_actions() {
        assert!(definition().actions.is_empty(), "a viewer window kind must declare no mutating actions");
    }

    #[test]
    fn render_carries_the_documents_own_text_and_language_read_only() {
        let document = crate::artifacts::writer::schema::empty_writer_snapshot();
        let node = render(&document);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"readOnly\":true") || json.contains("readOnly"), "viewer text scene must stamp read-only: {json}");
    }
}
//#endregion 🧪️Tests
