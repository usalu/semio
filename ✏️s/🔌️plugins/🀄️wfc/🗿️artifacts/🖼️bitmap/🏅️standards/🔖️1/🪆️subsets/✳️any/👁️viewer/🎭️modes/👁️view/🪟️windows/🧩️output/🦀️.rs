//! 🧩️ Bitmap viewer — `output` window: the PROBLEM's output surface, read-only. A viewer runs no
//! collapse: the solved bitmap is an inference an editor instance drives and caches, and rendering
//! one here would mean either running a full solve per frame or showing another surface's cache.
//! What this pane states instead is real and cheap — the declared output extent and every pinned
//! cell in its own palette colour, which is exactly the part of the answer the document itself
//! authors.

use crate::BitmapSnapshot;
use semio_framework_plugin::{scene_surface, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;

//#region 🔖️Constants
pub const WFC_BITMAP_VIEW_WINDOW_OUTPUT: &str = "wfc-bitmap-output";
pub const BODY_KEY: &str = "wfc.bitmap.output";
const SURFACE_ID: &str = "wfc.bitmap.view.output";
const VIEW_ZOOM: f64 = 8.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::bitmap::create_bitmap_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WFC_BITMAP_VIEW_WINDOW_OUTPUT.into(),
        label: LocalizedLabel::native("Output", "Ausgabe"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "grid-3x3".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Builds the read-only output surface and propagates assembly failures.
pub fn render(document: &BitmapSnapshot) -> UiAssemblyResult<BuiltNode> {
    let layers = crate::bitmap_layers_json("out", document.output.width, document.output.height, &document.input.palette, &[], &document.pinned);
    scene_surface(SURFACE_ID, ContractSurfaceKind::Canvas2d, &Canvas2dScene::base(0.0, 0.0, VIEW_ZOOM, layers))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
