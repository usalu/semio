//! ✒️ Writer viewer — Main window: read-only text view built on the framework `TextWindowKit`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6). Writer's artifact is
//! text-based, so the general-purpose text window kit fits without a bespoke render function —
//! no world-3d/mesh scene, no editor-only chrome (selection/tokens/diagnostics/completions all stay
//! on the sibling `editor` module's own window, never read from here).

use crate::{writer_text, WriterSnapshot};
use semio_framework_plugin::app::{TextView, TextWindowKit};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult, WindowKindDefinition, WindowKit};

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
pub fn render(document: &WriterSnapshot) -> UiAssemblyResult<BuiltNode> {
    TextWindowKit::render(&TextView { text: writer_text(document), language: Some(document.language_id.clone()), read_only: true })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
