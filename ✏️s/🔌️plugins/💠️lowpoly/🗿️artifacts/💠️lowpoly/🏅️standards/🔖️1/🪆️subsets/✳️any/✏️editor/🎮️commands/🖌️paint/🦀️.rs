//! 🖌️ Lowpoly play app commands — painting (`paintAt`/`paintStroke` from World3d picks, `canvasPointerDown`/
//! `canvasPointerMove`/`canvasPointerUp` from the UV canvas), one-shot fills (`paintFill`/`fillBucket`), sampling
//! (`paintSample`) and paint-layer creation (`addPaintLayer`).
//!
//! 🛠️ A press-drag-release is ONE paint gesture of the lowpoly TOOL (`🖌️session`'s `lowpoly_tool` under the
//! `🛠️tool-machine` runner): the brush and eraser yield ONE `apply-paint-stroke` whose dabs grow tick by tick, the fill
//! yields ONE `edit-paint-layer` at its press point. The open transaction lives in the artifact transient keyed by the
//! owning window (previewed by every window, never history) until the release commits it as one edit; a host abort
//! (`blur`, `captureLost`, `baseMoved`, …) leaves zero trace. A dispatch without `phase` is a one-shot gesture. The
//! eyedropper only samples into config (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::{lowpoly_paint_drive, lowpoly_tool_emit, paint_uv_from_command, LowpolyScratch, LowpolyTransientMutation};
use semio_framework_tool_machine::GesturePhase;
use crate::editor::lowpoly::view::{resolve_active_object_id, utility_param_f32, utility_params_value};
use crate::editor::lowpoly::LowpolyPlayApp;
use crate::mutations::{apply_paint_stroke::ApplyPaintStroke, edit_paint_layer::EditPaintLayer, PixelRun};
use crate::op::LowpolyMutation;
use crate::schema::{composite_layer_pixels, flood_fill, pixel_runs_from_diff, sample_pixel_from};
use crate::{LowpolyPaintLayer, LowpolySnapshot};
use semio_framework_plugin::app::ArtifactOwnedToolJobContext;
use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
use semio_framework_plugin::{AppOperationContext, ArtifactView, ConfigView, EditorApp, Emit, EphemeralEmit, Fault};

//#region 🖌️PaintTick
/// 🎯️ What one paint tick at `points` on `object_id` amounts to under the active paint utility: the leaf it yields
/// (a brush or eraser stroke of those dabs, a flood fill at the first point), the colour the eyedropper samples into
/// config, or nothing (no such object or layer, a brush the stroke leaf refuses).
pub enum LowpolyPaintTick {
    Leaf(LowpolyMutation),
    Sample(LowpolyConfigMutation),
    Nothing,
}

/// 🧮️ The tick of `points` (UV, v up) on `object_id` under `config`'s paint utility, brush and active layer.
pub fn lowpoly_paint_tick(projection: &LowpolySnapshot, config: &LowpolyConfig, object_id: &str, points: &[(f32, f32)]) -> LowpolyPaintTick {
    let unit = |value: f32| if value.is_finite() { value.clamp(0.0, 1.0) } else { 0.5 };
    let Some(object) = projection.objects.iter().find(|object| object.id == object_id) else { return LowpolyPaintTick::Nothing };
    let Some(&(u, v)) = points.first() else { return LowpolyPaintTick::Nothing };
    let layer_index = config.active_paint_layer as usize;
    let color = [config.paint_color_r, config.paint_color_g, config.paint_color_b, config.paint_color_a];
    match config.paint_utility.as_str() {
        "eyedropper" => {
            let sampled = sample_pixel_from(&composite_layer_pixels(&object.paint_layers), unit(u), unit(v));
            LowpolyPaintTick::Sample(LowpolyConfigMutation::SetPaintColor { r: sampled[0], g: sampled[1], b: sampled[2], a: sampled[3] })
        }
        "fill" => {
            let Some(layer) = object.paint_layers.get(layer_index) else { return LowpolyPaintTick::Nothing };
            let base = layer.materialized_pixels();
            let mut filled = base.clone();
            flood_fill(&mut filled, unit(u), unit(v), color);
            let runs: Vec<PixelRun> = pixel_runs_from_diff(&base, &filled).into_iter().map(|(offset, bytes)| PixelRun { offset, bytes }).collect();
            if runs.is_empty() {
                LowpolyPaintTick::Nothing
            } else {
                LowpolyPaintTick::Leaf(LowpolyMutation::EditPaintLayer(EditPaintLayer { object_id: object_id.to_string(), layer_index, runs }))
            }
        }
        utility => {
            if layer_index >= object.paint_layers.len() {
                return LowpolyPaintTick::Nothing;
            }
            let params = utility_params_value(config);
            let stroke = ApplyPaintStroke {
                object_id: object_id.to_string(),
                layer_index,
                eraser: utility == "eraser",
                color: [color[0], color[1], color[2]].map(|channel| f32::from(channel) / 255.0),
                radius: utility_param_f32(&params, "brushSize", 16.0),
                hardness: utility_param_f32(&params, "brushHardness", 0.5).clamp(0.0, 1.0),
                opacity: utility_param_f32(&params, "brushOpacity", 1.0).clamp(0.0, 1.0),
                points: points.iter().map(|(u, v)| [unit(*u), unit(*v)]).collect(),
            };
            if stroke.invariant_violation().is_some() {
                LowpolyPaintTick::Nothing
            } else {
                LowpolyPaintTick::Leaf(LowpolyMutation::ApplyPaintStroke(stroke))
            }
        }
    }
}

/// 🎨️ The UV point a paint command's fields name (`u`/`v`, else canvas `x`/`y`), and every canvas sample after it.
fn paint_points(u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32>, samples: &[[f32; 2]]) -> Vec<(f32, f32)> {
    let sampled: Vec<(f32, f32)> = samples.iter().filter_map(|sample| paint_uv_from_command(None, None, Some(sample[0]), Some(sample[1]))).collect();
    if sampled.is_empty() {
        paint_uv_from_command(u, v, x, y).into_iter().collect()
    } else {
        sampled
    }
}

/// 🛠️ The unmounted route of a paint verb (no retained context, so no persisted gesture): a one-shot or a commit at
/// rest is ONE tool transaction of its tick; a streamed phase needs the retained route's transient.
fn paint_once(verb: &str, phase: Option<&str>, reason: Option<&str>, object_id: Option<&str>, points: &[(f32, f32)], doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    if !matches!(GesturePhase::parse(phase, reason), Some(GesturePhase::Once | GesturePhase::Commit)) {
        return Err(Fault::from("lowpoly.paint.transient-context-required"));
    }
    let object_id = object_id.map_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot), str::to_string);
    Ok(match lowpoly_paint_tick(doc.snapshot, cfg.snapshot, &object_id, points) {
        LowpolyPaintTick::Leaf(leaf) => lowpoly_tool_emit(verb, doc, vec![leaf]),
        LowpolyPaintTick::Sample(mutation) => Emit::config(vec![mutation]),
        LowpolyPaintTick::Nothing => Emit::default(),
    })
}

/// 🛠️ The retained route of every paint verb: builds the dispatch's tick on the committed `snapshot`, drives the
/// dispatching window's paint tool through its phase against the gesture the transient holds for that window, and
/// publishes the committed transaction as ONE edit stamped with its ref plus the window's next transient. A stream
/// tick of a window with no open gesture opens one only when `opens` (a press), so a stray canvas move leaves nothing.
#[expect(clippy::too_many_arguments, reason = "One retained paint dispatch: its phase, reason, target and points, whether it may open a gesture, plus the retained inputs it reads.")]
pub fn lowpoly_paint_step(
    phase: Option<&str>,
    reason: Option<&str>,
    object_id: Option<&str>,
    points: &[(f32, f32)],
    opens: bool,
    snapshot: &LowpolySnapshot,
    config: &LowpolyConfig,
    context: &ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>,
    operation: &AppOperationContext,
) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
    let phase = GesturePhase::parse(phase, reason).ok_or_else(|| Fault::from("lowpoly.paint.phase-unknown"))?;
    let window = context.view_state.as_ref().and_then(|view| view.window_id.clone()).unwrap_or_default();
    if phase == GesturePhase::Stream && !opens && context.transient.paint(&window).is_none() {
        return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
    }
    let gesture_object = context.transient.paint(&window).and_then(|gesture| match &gesture.leaf {
        LowpolyMutation::ApplyPaintStroke(stroke) => Some(stroke.object_id.clone()),
        LowpolyMutation::EditPaintLayer(fill) => Some(fill.object_id.clone()),
        _ => None,
    });
    let object_id = object_id.map(str::to_string).or(gesture_object).unwrap_or_else(|| resolve_active_object_id(snapshot, config));
    let tick = match lowpoly_paint_tick(snapshot, config, &object_id, points) {
        LowpolyPaintTick::Sample(mutation) => return Ok(ArtifactCommandWorkStep::Complete(Emit::config(vec![mutation]))),
        LowpolyPaintTick::Leaf(leaf) => Some(leaf),
        LowpolyPaintTick::Nothing => None,
    };
    let base_revision: String = operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect();
    let drive = lowpoly_paint_drive(&context.transient, &window, phase, tick, &operation.authoring_seed, &base_revision);
    let emit = match drive.committed {
        Some((reference, mutations)) if !operation.authoring_seed.is_empty() => Emit::commit_transaction(reference, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    };
    Ok(match drive.transient {
        Some(transient) => ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }], window_transient: Vec::new() } },
        None => ArtifactCommandWorkStep::Complete(emit),
    })
}
//#endregion 🖌️PaintTick

//#region 🔖️PaintAt
pub mod paint_at {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "paint-at")]
    pub struct PaintAt {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
        pub phase: Option<String>,
        pub reason: Option<String>,
    }

    impl PaintAt {
        /// 🎨️ The dabs this dispatch names.
        pub fn points(&self) -> Vec<(f32, f32)> {
            paint_points(self.u, self.v, self.x, self.y, &[])
        }
    }

    pub fn handle(payload: &PaintAt, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        paint_once("paintAt", payload.phase.as_deref(), payload.reason.as_deref(), payload.object_id.as_deref(), &payload.points(), doc, cfg)
    }
}
//#endregion 🔖️PaintAt

//#region 🔖️PaintStroke
pub mod paint_stroke {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "paint-stroke")]
    pub struct PaintStroke {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
        pub phase: Option<String>,
        pub reason: Option<String>,
    }

    impl PaintStroke {
        /// 🎨️ The dabs this dispatch names.
        pub fn points(&self) -> Vec<(f32, f32)> {
            paint_points(self.u, self.v, self.x, self.y, &[])
        }
    }

    pub fn handle(payload: &PaintStroke, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        paint_once("paintStroke", payload.phase.as_deref(), payload.reason.as_deref(), payload.object_id.as_deref(), &payload.points(), doc, cfg)
    }
}
//#endregion 🔖️PaintStroke

//#region 🔖️CanvasPointerDown
pub mod canvas_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "canvas-pointer-down")]
    pub struct CanvasPointerDown {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    impl CanvasPointerDown {
        /// 🎨️ The press dab.
        pub fn points(&self) -> Vec<(f32, f32)> {
            paint_points(self.u, self.v, self.x, self.y, &[])
        }
    }

    pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        paint_once("canvasPointerDown", None, None, payload.object_id.as_deref(), &payload.points(), doc, cfg)
    }
}
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
pub mod canvas_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "canvas-pointer-move")]
    pub struct CanvasPointerMove {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
        pub samples: Option<Vec<[f32; 2]>>,
    }

    impl CanvasPointerMove {
        /// 🎨️ Every dab the move's samples (else its last point) name.
        pub fn points(&self) -> Vec<(f32, f32)> {
            paint_points(self.u, self.v, self.x, self.y, self.samples.as_deref().unwrap_or_default())
        }
    }

    pub fn handle(_payload: &CanvasPointerMove, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CanvasPointerMove

//#region 🔖️CanvasPointerUp
pub mod canvas_pointer_up {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "canvas-pointer-up")]
    pub struct CanvasPointerUp {
        pub cancelled: Option<bool>,
    }

    impl CanvasPointerUp {
        /// 🎚️ The release commits the open gesture; a cancelled one (capture lost, pointer left) aborts it.
        pub fn phase(&self) -> (&'static str, Option<&'static str>) {
            if self.cancelled == Some(true) { ("abort", Some("captureLost")) } else { ("commit", None) }
        }
    }

    pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CanvasPointerUp

//#region 🔖️PaintFill
pub mod paint_fill {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "paint-fill")]
    pub struct PaintFill {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintFill, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        fill_once("paintFill", payload.object_id.as_deref(), paint_uv_from_command(payload.u, payload.v, payload.x, payload.y), doc, cfg)
    }
}
//#endregion 🔖️PaintFill

//#region 🔖️FillBucket
pub mod fill_bucket {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "fill-bucket")]
    pub struct FillBucket {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &FillBucket, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        fill_once("fillBucket", payload.object_id.as_deref(), paint_uv_from_command(payload.u, payload.v, payload.x, payload.y), doc, cfg)
    }
}

/// 🪣️ A one-shot flood fill as ONE tool transaction of its `edit-paint-layer`. Without a canvas point (the Actions pane
/// row) the fill floods the region under the layer's centre.
fn fill_once(verb: &str, object_id: Option<&str>, point: Option<(f32, f32)>, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
    let object_id = object_id.map_or_else(|| resolve_active_object_id(doc.snapshot, cfg.snapshot), str::to_string);
    let fill = LowpolyConfig { paint_utility: "fill".into(), ..cfg.snapshot.clone() };
    Ok(match lowpoly_paint_tick(doc.snapshot, &fill, &object_id, &[point.unwrap_or((0.5, 0.5))]) {
        LowpolyPaintTick::Leaf(leaf) => lowpoly_tool_emit(verb, doc, vec![leaf]),
        LowpolyPaintTick::Sample(_) | LowpolyPaintTick::Nothing => Emit::default(),
    })
}
//#endregion 🔖️FillBucket

//#region 🔖️PaintSample
pub mod paint_sample {
    use super::*;

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[dsl(keyword = "paint-sample")]
    pub struct PaintSample {
        pub object_id: Option<String>,
        pub u: Option<f32>,
        pub v: Option<f32>,
        pub x: Option<f32>,
        pub y: Option<f32>,
    }

    pub fn handle(payload: &PaintSample, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let Some((uu, vv)) = paint_uv_from_command(payload.u, payload.v, payload.x, payload.y) else { return Ok(Emit::default()) };
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

    #[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
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
