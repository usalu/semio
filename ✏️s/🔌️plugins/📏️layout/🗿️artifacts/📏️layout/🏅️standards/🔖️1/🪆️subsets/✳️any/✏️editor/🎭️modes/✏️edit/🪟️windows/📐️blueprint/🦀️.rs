//! 📐️ Layout play app — the Blueprint window: the editable authoring surface with chrome (guides,
//! margins, dashed inherited-frame strokes) — the only window content-authoring actions are scoped to.

use crate::editor::layout::canvas::canvas_layers;
use crate::editor::layout::LayoutInteractionSnapshot;
use super::config::LayoutWindowConfig;
use super::transient::LayoutWindowTransient;
use crate::LayoutSnapshot;
use semio_framework_plugin::{Canvas2dScene, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

#[path = "🪛️utilities/🖱️select/🦀️.rs"]
pub mod select;
#[path = "🪛️utilities/🔄️transform/🦀️.rs"]
pub mod transform;

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
        utilities: vec![select::UTILITY_ID.into(), transform::UTILITY_ID.into()],
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
pub fn render(doc: &LayoutSnapshot, config: &LayoutWindowConfig, transient: &LayoutWindowTransient, interaction: &LayoutInteractionSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let camera = &config.camera;
    semio_framework_plugin::scene_surface(
        LAYOUT_PLAY_SURFACE_BLUEPRINT,
        semio_framework_ui_contract::SurfaceKind::Canvas2d,
        &Canvas2dScene { camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom, layers_json: canvas_layers(doc, config, transient, interaction, true), snapshot: None, tool_run_trace: None, lanes: Vec::new() },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
