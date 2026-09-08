//! 🖌️ Lowpoly play app commands — paint stroke lifecycle (`paintStrokeBegin`/`paintStroke`/`paintAt`/
//! `canvasPointerDown`/`canvasPointerMove`/`paintStrokeEnd`), single-shot fill (`paintFill`/
//! `fillBucket`), sampling (`paintSample`) and paint-layer creation (`addPaintLayer`).

use crate::op::LowpolyMutation;
use crate::schema::{composite_layer_pixels, sample_pixel_from};
use crate::{LowpolyPaintLayer, LowpolySnapshot};
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::editor::lowpoly::view::resolve_active_object_id;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 🎯️ Extracts UV (0..1) from a paint command's fields — either direct `u`/`v` (world 3d picks) or
/// canvas `x`/`y` positions mapped through the paint-texture extent (UV canvas).
fn paint_uv(u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32>) -> Option<(f32, f32)> {
    crate::editor::lowpoly::session::paint_uv_from_command(u, v, x, y)
}

/// 🎯️ Shared body for `PaintStroke`/`PaintAt`/`CanvasPointerDown` — identical field shape, distinct
/// wire keywords (mirrors the old ui crate's single grouped match arm before the taxonomy split).
/// Bare `Emit` (no `Result`): every one of its 3 call sites is a handler's tail expression, wrapped in
/// `Ok(...)` there to satisfy `app_commands!`'s `Result<Emit<_, _>, Fault>` handler signature.
#[allow(clippy::too_many_arguments, reason = "1:1 forwarder for the 3 identically-shaped paint-tick commands' fields (object_id + u/v/x/y); a params struct would only move the same fields around for this one shared body")]
fn paint_tick_command(
    doc: &ArtifactView<'_, LowpolySnapshot>,
    cfg: &ConfigView<'_, LowpolyConfig>,
    ctx: &mut LowpolyScratch,
    object_id: Option<String>,
    u: Option<f32>,
    v: Option<f32>,
    x: Option<f32>,
    y: Option<f32>,
) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    let Some((uu, vv)) = paint_uv(u, v, x, y) else { return Emit::default() };
    let object_id = object_id.unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
    ctx.paint_tick(doc.snapshot, cfg.snapshot, &object_id, uu, vv)
}

//#region 🔖️PaintStrokeBegin
pub mod paint_stroke_begin {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "paint-stroke-begin")]
    pub struct PaintStrokeBegin {}

    pub fn handle(_payload: &PaintStrokeBegin, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        ctx.begin_stroke_drag();
        Ok(Emit::default())
    }
}
//#endregion 🔖️PaintStrokeBegin

//#region 🔖️PaintStrokeEnd
pub mod paint_stroke_end {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "paint-stroke-end")]
    pub struct PaintStrokeEnd {}

    pub fn handle(_payload: &PaintStrokeEnd, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(ctx.end_stroke_drag())
    }
}
//#endregion 🔖️PaintStrokeEnd

//#region 🔖️PaintStroke
pub mod paint_stroke {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "paint-stroke")]
    pub struct PaintStroke {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintStroke, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(paint_tick_command(doc, cfg, ctx, payload.object_id.clone(), payload.u, payload.v, payload.x, payload.y))
    }
}
//#endregion 🔖️PaintStroke

//#region 🔖️PaintAt
pub mod paint_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "paint-at")]
    pub struct PaintAt {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintAt, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(paint_tick_command(doc, cfg, ctx, payload.object_id.clone(), payload.u, payload.v, payload.x, payload.y))
    }
}
//#endregion 🔖️PaintAt

//#region 🔖️CanvasPointerDown
pub mod canvas_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "canvas-pointer-down")]
    pub struct CanvasPointerDown {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(paint_tick_command(doc, cfg, ctx, payload.object_id.clone(), payload.u, payload.v, payload.x, payload.y))
    }
}
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
pub mod canvas_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "canvas-pointer-move")]
    pub struct CanvasPointerMove {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
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

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "paint-fill")]
    pub struct PaintFill {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintFill, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let Some((uu, vv)) = paint_uv(payload.u, payload.v, payload.x, payload.y) else { return Ok(Emit::default()) };
        let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
        Ok(ctx.fill_at(doc.snapshot, cfg.snapshot, object_id, uu, vv))
    }
}
//#endregion 🔖️PaintFill

//#region 🔖️FillBucket
pub mod fill_bucket {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "fill-bucket")]
    pub struct FillBucket {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &FillBucket, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let Some((uu, vv)) = paint_uv(payload.u, payload.v, payload.x, payload.y) else { return Ok(Emit::default()) };
        let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
        Ok(ctx.fill_at(doc.snapshot, cfg.snapshot, object_id, uu, vv))
    }
}
//#endregion 🔖️FillBucket

//#region 🔖️PaintSample
pub mod paint_sample {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "paint-sample")]
    pub struct PaintSample {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintSample, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
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

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "add-paint-layer")]
    pub struct AddPaintLayer {
        pub object_id: Option<String>,
        pub name: Option<String>,
    }

    pub fn handle(payload: &AddPaintLayer, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot));
        let name = payload.name.as_deref().unwrap_or("Layer");
        let index = doc.snapshot.objects.iter().find(|object| object.id == object_id).map_or(0, |object| object.paint_layers.len());
        Ok(Emit::mutations(vec![LowpolyMutation::InsertPaintLayer(crate::mutations::insert_paint_layer::InsertPaintLayer { object_id, index, layer: LowpolyPaintLayer::new(name) })]))
    }
}
//#endregion 🔖️AddPaintLayer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
