//! 🖌️ Lowpoly play app commands — paint stroke lifecycle (`paintStrokeBegin`/`paintStroke`/`paintAt`/
//! `canvasPointerDown`/`canvasPointerMove`/`paintStrokeEnd`), single-shot fill (`paintFill`/
//! `fillBucket`), sampling (`paintSample`) and paint-layer creation (`addPaintLayer`).

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::apps::lowpoly::view::resolve_active_object_id;
use crate::artifacts::lowpoly::engine::{composite_layer_pixels, sample_pixel_from};
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::{LowpolyPaintLayer, LowpolySnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🎯️ Extracts UV (0..1) from a paint command's fields — either direct `u`/`v` (world 3d picks) or
/// canvas `x`/`y` positions mapped through the paint-texture extent (UV canvas).
fn paint_uv(u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32>) -> Option<(f32, f32)> {
    crate::apps::lowpoly::session::paint_uv_from_command(u, v, x, y)
}

/// 🎯️ Shared body for `PaintStroke`/`PaintAt`/`CanvasPointerDown` — identical field shape, distinct
/// wire keywords (mirrors the old ui crate's single grouped match arm before the taxonomy split).
/// Bare `Emit` (no `Result`): every one of its 3 call sites is a handler's tail expression, wrapped in
/// `Ok(...)` there to satisfy `app_commands!`'s `Result<Emit<_, _>, Fault>` handler signature.
#[allow(clippy::too_many_arguments, reason = "1:1 forwarder for the 3 identically-shaped paint-tick commands' fields (object_id + u/v/x/y); a params struct would only move the same fields around for this one shared body")]
fn paint_tick_command(doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch, object_id: Option<String>, u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32>) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    let Some((uu, vv)) = paint_uv(u, v, x, y) else { return Emit::default() };
    let object_id = object_id.unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
    ctx.paint_tick(doc.snapshot, cfg.snapshot, &object_id, uu, vv)
}

//#region 🔖️PaintStrokeBegin
pub mod paint_stroke_begin {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "paint-stroke-begin")]
    pub struct PaintStrokeBegin {}

    pub fn handle(_payload: &PaintStrokeBegin, _doc: &DocumentView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        ctx.begin_stroke_drag();
        Ok(Emit::default())
    }
}
//#endregion 🔖️PaintStrokeBegin

//#region 🔖️PaintStrokeEnd
pub mod paint_stroke_end {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "paint-stroke-end")]
    pub struct PaintStrokeEnd {}

    pub fn handle(_payload: &PaintStrokeEnd, _doc: &DocumentView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(ctx.end_stroke_drag())
    }
}
//#endregion 🔖️PaintStrokeEnd

//#region 🔖️PaintStroke
pub mod paint_stroke {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "paint-stroke")]
    pub struct PaintStroke {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintStroke, doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(paint_tick_command(doc, cfg, ctx, payload.object_id.clone(), payload.u, payload.v, payload.x, payload.y))
    }
}
//#endregion 🔖️PaintStroke

//#region 🔖️PaintAt
pub mod paint_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "paint-at")]
    pub struct PaintAt {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintAt, doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(paint_tick_command(doc, cfg, ctx, payload.object_id.clone(), payload.u, payload.v, payload.x, payload.y))
    }
}
//#endregion 🔖️PaintAt

//#region 🔖️CanvasPointerDown
pub mod canvas_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-down")]
    pub struct CanvasPointerDown {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &CanvasPointerDown, doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(paint_tick_command(doc, cfg, ctx, payload.object_id.clone(), payload.u, payload.v, payload.x, payload.y))
    }
}
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
pub mod canvas_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-move")]
    pub struct CanvasPointerMove {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &CanvasPointerMove, doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        if !ctx.stroke_drag_active() {
            return Ok(Emit::default());
        }
        Ok(paint_tick_command(doc, cfg, ctx, payload.object_id.clone(), payload.u, payload.v, payload.x, payload.y))
    }
}
//#endregion 🔖️CanvasPointerMove

//#region 🔖️PaintFill
pub mod paint_fill {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "paint-fill")]
    pub struct PaintFill {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintFill, doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let Some((uu, vv)) = paint_uv(payload.u, payload.v, payload.x, payload.y) else { return Ok(Emit::default()) };
        let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
        Ok(ctx.fill_at(doc.snapshot, cfg.snapshot, object_id, uu, vv))
    }
}
//#endregion 🔖️PaintFill

//#region 🔖️FillBucket
pub mod fill_bucket {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "fill-bucket")]
    pub struct FillBucket {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &FillBucket, doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let Some((uu, vv)) = paint_uv(payload.u, payload.v, payload.x, payload.y) else { return Ok(Emit::default()) };
        let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
        Ok(ctx.fill_at(doc.snapshot, cfg.snapshot, object_id, uu, vv))
    }
}
//#endregion 🔖️FillBucket

//#region 🔖️PaintSample
pub mod paint_sample {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "paint-sample")]
    pub struct PaintSample {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintSample, doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let Some((uu, vv)) = paint_uv(payload.u, payload.v, payload.x, payload.y) else { return Ok(Emit::default()) };
        let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
        let Some(object) = doc.snapshot.objects.iter().find(|object| object.id == object_id) else { return Ok(Emit::default()) };
        let composite = composite_layer_pixels(&object.paint_layers);
        let color = sample_pixel_from(&composite, uu, vv);
        Ok(Emit::config(vec![LowpolyConfigMutation::SetPaintColor { r: color[0], g: color[1], b: color[2], a: color[3] }]))
    }
}
//#endregion 🔖️PaintSample

//#region 🔖️AddPaintLayer
pub mod add_paint_layer {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-paint-layer")]
    pub struct AddPaintLayer {
        pub object_id: Option<String>,
        pub name: Option<String>,
    }

    pub fn handle(payload: &AddPaintLayer, doc: &DocumentView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
        let name = payload.name.as_deref().unwrap_or("Layer");
        let index = doc.snapshot.objects.iter().find(|object| object.id == object_id).map_or(0, |object| object.paint_layers.len());
        Ok(Emit::mutations(vec![LowpolyMutation::AddPaintLayer { object_id, index, layer: LowpolyPaintLayer::new(name) }]))
    }
}
//#endregion 🔖️AddPaintLayer

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;
    use semio_framework_plugin::{testkit, PluginApp};

    #[test]
    fn add_paint_layer_emits_operation() {
        let mut a = app();
        let before = a.snapshot().expect("projection").objects[0].paint_layers.len();
        dispatch(&mut a, LowpolyCommand::AddPaintLayer(super::add_paint_layer::AddPaintLayer { object_id: None, name: Some("Detail".into()) }));
        assert_eq!(a.snapshot().expect("projection").objects[0].paint_layers.len(), before + 1);
    }

    #[test]
    fn paint_stroke_drag_is_one_undo_step_with_pixel_restoration() {
        let mut a = app();
        let object_id = a.snapshot().expect("projection").objects[0].id.clone();
        let before = a.snapshot().expect("projection").objects[0].paint_layers[0].pixels.clone();
        // begin → tick → tick → end : one undoable PaintStroke edit.
        a.dispatch_typed(LowpolyCommand::PaintStrokeBegin(super::paint_stroke_begin::PaintStrokeBegin {}), &testkit::meta("a")).unwrap();
        let tick_a = a.dispatch_typed(LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: Some(object_id.clone()), u: Some(0.5), v: Some(0.5), x: None, y: None }), &testkit::meta("a")).unwrap();
        let tick_b = a.dispatch_typed(LowpolyCommand::PaintAt(super::paint_at::PaintAt { object_id: Some(object_id), u: Some(0.52), v: Some(0.5), x: None, y: None }), &testkit::meta("a")).unwrap();
        assert!(tick_a.mutations.is_empty() && tick_b.mutations.is_empty(), "mid-drag ticks emit no operations");
        let end = a.dispatch_typed(LowpolyCommand::PaintStrokeEnd(super::paint_stroke_end::PaintStrokeEnd {}), &testkit::meta("a")).unwrap();
        assert_eq!(end.mutations.len(), 1, "the whole drag commits as one operation");
        let painted = a.snapshot().expect("projection").objects[0].paint_layers[0].pixels.clone();
        assert_ne!(painted, before, "the stroke changed pixels");
        a.handle_action("undo", None, &testkit::meta("a")).unwrap();
        let restored = a.snapshot().expect("projection").objects[0].paint_layers[0].pixels.clone();
        assert_eq!(restored, before, "undo restores the exact pre-stroke pixels");
        a.handle_action("redo", None, &testkit::meta("a")).unwrap();
        assert_eq!(a.snapshot().expect("projection").objects[0].paint_layers[0].pixels, painted);
    }

    #[test]
    fn eyedropper_updates_paint_color_without_operations() {
        let mut a = app();
        // 🧰️ The host-owned utility switch bridges into config.paint_utility and emits no operations.
        let switch = a.dispatch_typed(LowpolyCommand::SetActiveUtility(crate::apps::lowpoly::commands::utility::set_active_utility::SetActiveUtility { utility_id: "eyedropper".into() }), &testkit::meta("a")).unwrap();
        assert!(switch.mutations.is_empty());
        let result = a.dispatch_typed(LowpolyCommand::PaintSample(super::paint_sample::PaintSample { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }), &testkit::meta("a")).unwrap();
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
