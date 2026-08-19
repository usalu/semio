//! 🖱️ 🖱️ Layout play app commands command — `canvas-drag-over`.

use crate::editor::layout::canvas::active_page;
use crate::editor::layout::commands::{add_frame, add_page};
use crate::editor::layout::config::LayoutConfig;
use crate::artifacts::layout::LayoutDropPreviewState;
use crate::editor::layout::config::LayoutConfigMutation;
use crate::editor::layout::engine::scene::{build_display_list_for_page, LayoutEngine};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutCamera, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Shared
/// 🖱️ A surface id names its blueprint/preview surface directly (`"layout.play.blueprint"` /
/// `"layout.play.preview"`); an absent id defaults to blueprint (the interactive authoring surface).
async fn surface_is_blueprint(surface_id: Option<&str>) -> bool {
    surface_id.is_none_or(|surface| surface.contains("blueprint"))
}

async fn screen_to_world_for_surface(config: &LayoutConfig, blueprint: bool, sx: f64, sy: f64, width: f64, height: f64) -> (f64, f64) {
    let camera_runtime = if blueprint { &config.camera } else { &config.preview_camera };
    let camera = infinite_canvas::camera::Camera { x: camera_runtime.x, y: camera_runtime.y, zoom: camera_runtime.zoom.max(0.0001) };
    let viewport = infinite_canvas::camera::Viewport { width: width.max(1.0) as u32, height: height.max(1.0) as u32, dpr: 1.0 };
    let world = infinite_canvas::camera::screen_to_world(&camera, &viewport, infinite_canvas::Point::new(sx, sy));
    (world.x, world.y)
}

#[allow(clippy::too_many_arguments)]
async fn hit_test_at(doc: &LayoutSnapshot, config: &LayoutConfig, sx: f64, sy: f64, width: f64, height: f64, blueprint: bool) -> Option<String> {
    let page = active_page(doc, config)?;
    let (wx, wy) = screen_to_world_for_surface(config, blueprint, sx, sy, width, height);
    let mut engine = LayoutEngine::new();
    let list = build_display_list_for_page(&mut engine, doc, page, &page.id, &[], None, blueprint);
    list.hit_test(wx as f32, wy as f32)
}
//#endregion 🔖️Shared

//#region 🔖️CanvasPointerDown
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
//#endregion 🔖️CanvasPointerMove

//#region 🔖️CanvasPointerUp
//#endregion 🔖️CanvasPointerUp

//#region 🔖️CanvasDragOver
//#endregion 🔖️CanvasDragOver

//#region 🔖️CanvasDragLeave
//#endregion 🔖️CanvasDragLeave

//#region 🔖️SetCamera
//#endregion 🔖️SetCamera

//#region 🔖️CanvasDrop
//#endregion 🔖️CanvasDrop

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-drag-over")]
pub struct CanvasDragOver {
    pub surface_id: Option<String>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub async fn handle(payload: &CanvasDragOver, _doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
    if !blueprint {
        return Ok(Emit::default());
    }
    let (wx, wy) = screen_to_world_for_surface(cfg.snapshot, blueprint, payload.x, payload.y, payload.width, payload.height);
    Ok(Emit::config(vec![LayoutConfigMutation::SetDropPreview { preview: LayoutDropPreviewState { kind: payload.kind.clone(), x: wx, y: wy } }]))
}
