//! 🖱️ 🖱️ Layout play app commands command — `canvas-pointer-move`.
//!
//! 🎯️ Batched (design L4 / §2 D): the host folds every DOM `pointermove` of one turn into ONE
//! command whose `samples` carry every canvas-pixel position oldest-first; `x`/`y` stay the LAST
//! sample. Layout has no drag gesture on this surface — a move is a hover hit-test only — so the
//! batch collapses to its last sample (intermediate hovers are moot once a newer one exists).

use crate::editor::layout::canvas::active_page;
use crate::editor::layout::modes::edit::windows::blueprint::config::{current, LayoutWindowConfig};
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::layout::engine::scene::{build_display_list_for_page, LayoutEngine};
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🖱️ A surface id names its blueprint/preview surface directly (`"layout.play.blueprint"` /
/// `"layout.play.preview"`); an absent id defaults to blueprint (the interactive authoring surface).
fn surface_is_blueprint(surface_id: Option<&str>) -> bool {
    surface_id.is_none_or(|surface| surface.contains("blueprint"))
}

fn screen_to_world_for_surface(config: &LayoutWindowConfig, sx: f64, sy: f64, width: f64, height: f64) -> (f64, f64) {
    let camera_runtime = &config.camera;
    let camera = infinite_canvas::camera::Camera { x: camera_runtime.x, y: camera_runtime.y, zoom: camera_runtime.zoom.max(0.0001) };
    let viewport = infinite_canvas::camera::Viewport { width: width.max(1.0) as u32, height: height.max(1.0) as u32, dpr: 1.0 };
    let world = infinite_canvas::camera::screen_to_world(&camera, &viewport, infinite_canvas::Point::new(sx, sy));
    (world.x, world.y)
}

#[allow(clippy::too_many_arguments)]
fn hit_test_at(doc: &LayoutSnapshot, config: &LayoutWindowConfig, sx: f64, sy: f64, width: f64, height: f64) -> Option<String> {
    let page = active_page(doc, config)?;
    let (wx, wy) = screen_to_world_for_surface(config, sx, sy, width, height);
    let mut engine = LayoutEngine::new();
    // 🕹️ `selected_ids`/`hovered_id` only feed `DisplayRect.selected`/`.hovered` chrome flags, never
    // hit-test correctness — `&[]`/`None` here are harmless (selection/hover are framework-owned now).
    let list = build_display_list_for_page(&mut engine, doc, page, &page.id, &[], None, true);
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
pub struct CanvasPointerMove {
    pub surface_id: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first. Empty on a legacy
    /// (unbatched) wire — [`CanvasPointerMove::last_sample`] then yields `[x, y]`.
    #[value(default)]
    pub samples: Vec<[f64; 2]>,
}

impl CanvasPointerMove {
    /// 🧵️ The batch as canvas samples, oldest first — never empty: an absent/empty `samples`
    /// degrades to the single `[x, y]`.
    pub fn samples_or_last(&self) -> Vec<[f64; 2]> {
        if self.samples.is_empty() {
            vec![[self.x, self.y]]
        } else {
            self.samples.clone()
        }
    }

    /// 🧵️ The newest sample of the batch (`[x, y]` on a legacy wire).
    pub fn last_sample(&self) -> [f64; 2] {
        self.samples.last().copied().unwrap_or([self.x, self.y])
    }
}

pub fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let blueprint = surface_is_blueprint(payload.surface_id.as_deref());
    if !blueprint {
        return Ok(Emit::default());
    }
    let [x, y] = payload.last_sample();
    let hit = hit_test_at(doc.snapshot, &current(cfg), x, y, payload.width, payload.height);
    Ok(Emit::effect(crate::editor::layout::layout_hover_effect(hit.as_deref())))
}
