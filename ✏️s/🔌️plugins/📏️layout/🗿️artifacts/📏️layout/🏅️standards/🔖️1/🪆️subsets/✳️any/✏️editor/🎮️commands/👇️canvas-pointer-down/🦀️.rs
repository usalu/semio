//! 🖱️ 🖱️ Layout play app commands command — `canvas-pointer-down`.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
#[cfg(test)]
use crate::LayoutCamera;
use crate::editor::layout::canvas::active_page;
use crate::editor::layout::config::LayoutConfig;
use crate::editor::layout::config::LayoutConfigMutation;
use crate::editor::layout::engine::scene::{build_display_list_for_page, LayoutEngine};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `dispatch_action` intercepts the six
// framework interaction verbs BEFORE routing to `ArtifactApp::handle`, so `LayoutCommand::dispatch`
// can no longer emit a selection mutation of its own; the hit test below is unchanged, only its
// result now travels as a `crate::editor::layout::layout_select_effect`/`layout_clear_selection_effect`
// redispatch instead of a `LayoutConfigMutation::SetSelection`.

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

#[allow(clippy::too_many_arguments)]
fn hit_test_at(doc: &LayoutSnapshot, config: &LayoutConfig, sx: f64, sy: f64, width: f64, height: f64, blueprint: bool) -> Option<String> {
    let page = active_page(doc, config)?;
    let (wx, wy) = screen_to_world_for_surface(config, blueprint, sx, sy, width, height);
    let mut engine = LayoutEngine::new();
    // 🕹️ `selected_ids`/`hovered_id` only feed `DisplayRect.selected`/`.hovered` chrome flags, never
    // hit-test correctness — `&[]`/`None` here are harmless (selection/hover are framework-owned now).
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-down")]
pub struct CanvasPointerDown {
    pub surface_id: Option<String>,
    pub button: i64,
    pub extend: bool,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
    if !blueprint || payload.button != 0 {
        return Ok(Emit::default());
    }
    let hit = hit_test_at(doc.snapshot, cfg.snapshot, payload.x, payload.y, payload.width, payload.height, blueprint);
    let effect = match hit {
        // ⚖️ `Invertive` merge is the toggle-on-shift-click `extend` used to hand-roll (add if absent,
        // remove if present); `Replace` is a plain click. Both now resolve against the CURRENT
        // framework-owned selection inside `dispatch_interaction_action`, not a locally-read snapshot.
        Some(id) => crate::editor::layout::layout_select_effect(std::slice::from_ref(&id), if payload.extend { "invertive" } else { "replace" }),
        None => crate::editor::layout::layout_clear_selection_effect(),
    };
    Ok(Emit::effect(effect))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
