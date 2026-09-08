//! 👁️ Layout play app — the Preview window: a read-only render of the current page with no chrome
//! (no guides, no dashed inherited-frame strokes) and its own independent camera pose.

use crate::LayoutSnapshot;
use crate::editor::layout::canvas::canvas_layers;
use crate::editor::layout::config::LayoutConfig;
use semio_framework_plugin::{Canvas2dScene, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const LAYOUT_PLAY_WINDOW_PREVIEW: &str = "layout-preview";
pub const LAYOUT_PLAY_BODY_PREVIEW: &str = "layout.play.preview";
pub const LAYOUT_PLAY_SURFACE_PREVIEW: &str = "layout.play.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::layout::create_layout_app`. `options.measures`
/// stays empty: layout declares no config-derived chrome measures for this window.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: LAYOUT_PLAY_WINDOW_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: LAYOUT_PLAY_BODY_PREVIEW.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "preview".into(),
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
pub fn render(engine: &mut crate::editor::layout::engine::scene::LayoutEngine, doc: &LayoutSnapshot, config: &LayoutConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let camera = &config.preview_camera;
    semio_framework_plugin::scene_surface(LAYOUT_PLAY_SURFACE_PREVIEW, semio_framework_ui_contract::SurfaceKind::Canvas2d, &Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: canvas_layers(engine, doc, config, false), snapshot: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
