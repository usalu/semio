//! 🖼️ Bitmap viewer — `input` window: the authored sample, read-only. An independent render from
//! the sibling mutation-capable surface — the same document fields read through the artifact root's
//! own shared layer builder, no edit affordances, and no import through `crate::editor`.

use crate::BitmapSnapshot;
use semio_framework_plugin::{scene_surface, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;

//#region 🔖️Constants
pub const WFC_BITMAP_VIEW_WINDOW_INPUT: &str = "wfc-bitmap-input";
pub const BODY_KEY: &str = "wfc.bitmap.input";
const SURFACE_ID: &str = "wfc.bitmap.view.input";
/// 🔎️ The viewer has no per-window configuration, so its canvas zoom is a stated constant rather
/// than a silent `1.0` that would render a 16×16 sample as sixteen pixels.
const VIEW_ZOOM: f64 = 12.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::bitmap::create_bitmap_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WFC_BITMAP_VIEW_WINDOW_INPUT.into(),
        label: LocalizedLabel::native("Input", "Eingabe"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "image".into(),
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
/// 👁️ Builds the read-only sample canvas and propagates assembly failures.
pub fn render(document: &BitmapSnapshot) -> UiAssemblyResult<BuiltNode> {
    let indices = document.input.indices().unwrap_or_default();
    let layers = crate::bitmap_layers_json("in", document.input.width, document.input.height, &document.input.palette, &indices, &[]);
    scene_surface(SURFACE_ID, ContractSurfaceKind::Canvas2d, &Canvas2dScene::base(0.0, 0.0, VIEW_ZOOM, layers))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
