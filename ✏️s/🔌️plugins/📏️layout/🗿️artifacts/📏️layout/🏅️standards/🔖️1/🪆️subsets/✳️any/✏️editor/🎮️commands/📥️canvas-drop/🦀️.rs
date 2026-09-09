//! 🖱️ 🖱️ Layout play app commands command — `canvas-drop`.

use crate::editor::layout::commands::{add_frame, add_page};
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::config::LayoutConfigMutation;
use crate::mutations::LayoutMutation;
use crate::LayoutDropPreviewState;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🖱️ A surface id names its blueprint/preview surface directly (`"layout.play.blueprint"` /
/// `"layout.play.preview"`); an absent id defaults to blueprint (the interactive authoring surface).
fn surface_is_blueprint(surface_id: Option<&str>) -> bool {
    surface_id.is_none_or(|surface| surface.contains("blueprint"))
}

fn screen_to_world_for_surface(config: &LayoutConfig, blueprint: bool, sx: f64, sy: f64, width: f64, height: f64) -> (f64, f64) {
    let camera_runtime = if blueprint { &config.camera } else { &config.preview_camera };
    let camera = infinite_canvas::camera::Camera { x: camera_runtime.x, y: camera_runtime.y, zoom: camera_runtime.zoom.max(0.0001) };
    let viewport = infinite_canvas::camera::Viewport { width: width.max(1.0) as u32, height: height.max(1.0) as u32, dpr: 1.0 };
    let world = infinite_canvas::camera::screen_to_world(&camera, &viewport, infinite_canvas::Point::new(sx, sy));
    (world.x, world.y)
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-drop")]
pub struct CanvasDrop {
    pub surface_id: Option<String>,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// 🐛️ Delegates document creation to `add_page`/`add_frame`'s own handlers so "drop adds content"
/// has one implementation, then always clears the drag-ghost regardless of surface/outcome.
pub fn handle(payload: &CanvasDrop, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
    if !blueprint {
        return Ok(Emit::config(vec![LayoutConfigMutation::SetDropPreview(crate::editor::layout::config::SetDropPreview { preview: LayoutDropPreviewState::default() })]));
    }
    let (wx, wy) = screen_to_world_for_surface(cfg.snapshot, blueprint, payload.x, payload.y, payload.width, payload.height);
    let mut emitted = if payload.kind == "page" { add_page::handle(&add_page::AddPage {}, doc, cfg)? } else { add_frame::handle(&add_frame::AddFrame { kind: payload.kind.clone(), x: Some(wx), y: Some(wy) }, doc, cfg)? };
    emitted.config_mutations.push(LayoutConfigMutation::SetDropPreview(crate::editor::layout::config::SetDropPreview { preview: LayoutDropPreviewState::default() }));
    Ok(emitted)
}
