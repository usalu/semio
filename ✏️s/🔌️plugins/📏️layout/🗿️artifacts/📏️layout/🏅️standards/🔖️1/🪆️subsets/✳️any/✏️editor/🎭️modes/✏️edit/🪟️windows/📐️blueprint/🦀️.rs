//! 📐️ Layout play app — the Blueprint window: the editable authoring surface with chrome (guides,
//! margins, dashed inherited-frame strokes) — the only window content-authoring actions are scoped to.

use crate::editor::layout::canvas::canvas_layers;
use crate::editor::layout::config::LayoutConfig;
use crate::LayoutSnapshot;
use semio_framework_plugin::{Canvas2dScene, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const LAYOUT_PLAY_WINDOW_BLUEPRINT: &str = "layout-blueprint";
pub const LAYOUT_PLAY_BODY_BLUEPRINT: &str = "layout.play.blueprint";
pub const LAYOUT_PLAY_SURFACE_BLUEPRINT: &str = "layout.play.blueprint";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::layout::create_layout_app`. `options.measures`
/// stays empty: layout declares no config-derived chrome measures for this window.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: LAYOUT_PLAY_WINDOW_BLUEPRINT.into(),
        label: LocalizedLabel::native("Blueprint", "Entwurf"),
        body_key: LAYOUT_PLAY_BODY_BLUEPRINT.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "layout".into(),
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
    let camera = &config.camera;
    semio_framework_plugin::scene_surface(
        LAYOUT_PLAY_SURFACE_BLUEPRINT,
        semio_framework_ui_contract::SurfaceKind::Canvas2d,
        &Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: canvas_layers(engine, doc, config, true), snapshot: None },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
