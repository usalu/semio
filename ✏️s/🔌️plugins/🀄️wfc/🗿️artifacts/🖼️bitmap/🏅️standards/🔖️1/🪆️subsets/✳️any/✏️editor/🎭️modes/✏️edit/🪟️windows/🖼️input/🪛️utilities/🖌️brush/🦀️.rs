//! 🖌️ Input-window utility — Brush: paints the input sample with the window's armed palette colour. It is also the
//! brush TOOL: a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the `🛠️tool-machine` runner. A
//! stroke is ONE `ToolTransaction` holding ONE parametric `paint-input-stroke` leaf — the sampled cells in drawing
//! order and the colour — whether it arrives in one dispatch (`paint-stroke` without a phase) or streams over many
//! (`phase: stream`, then `commit`). Between dispatches the open transaction lives in the window's ONE gesture slot of
//! the runtime (design §22.10): the runtime paints committed ⊕ provisional and ends the stroke with zero trace on every
//! host fact. Tool state is never history; the committed leaf is (design §5, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
#![allow(unexpected_cfgs)]

use crate::mutations::{stroke_extent, BitmapMutation, BitmapStrokePoint, BITMAP_STROKE_MAXIMUM_POINTS};
use crate::schema::mutations::paint_input_stroke::PaintInputStroke;
use crate::BitmapSnapshot;
use machine::Command;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_tool_machine::{ChartGesture, GestureChart, GesturePhase, ToolAbortReason, ToolYield};

pub const UTILITY_ID: &str = "brush";

/// 🏷️ The verb every brush stroke dispatches, and so the `tool` half of its transactions: `<appId>#paint-stroke`.
pub const BITMAP_PAINT_STROKE_VERB: &str = "paint-stroke";

/// 🔑️ The transaction key of a stroke's leaf — every tick upserts it, so an open stroke holds ONE net leaf.
pub const BITMAP_BRUSH_TOOL_KEY: &str = "stroke:0";

/// 🧱️ Stitched into the input window kind by `super::super::definition`.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Brush", "Pinsel"), "paintbrush") }
}

//#region 🛠️BrushTool
/// 📨️ One brush event: the stroke points this dispatch carries and the colour, plus the extent of the committed
/// sample they land on — dispatch inputs, never tool state.
#[derive(Clone, Debug)]
pub struct BrushToolRequest {
    pub stroke: PaintInputStroke,
    pub width: u32,
    pub height: u32,
}

impl BrushToolRequest {
    /// 🧮️ The request for `points` in `color` on `base`.
    pub fn on(base: &BitmapSnapshot, points: Vec<BitmapStrokePoint>, color: u32) -> Self {
        Self { stroke: PaintInputStroke { points, color }, width: base.input.width, height: base.input.height }
    }

    /// 🎯️ Whether `stroke` paints at least one cell of the sample.
    fn lands(&self, stroke: &PaintInputStroke) -> bool {
        !stroke.points.is_empty() && stroke_extent(&stroke.points, self.width, self.height).is_some()
    }
}

/// 🧰️ The brush tool's context: the stroke a streamed gesture has accumulated so far (`None` at rest).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BrushToolContext {
    pub stroke: Option<PaintInputStroke>,
}

fn brush_tool_context(input: BrushToolContext) -> BrushToolContext {
    input
}

/// ➕️ `stroke` followed by `tick`'s points, up to [`BITMAP_STROKE_MAXIMUM_POINTS`]; a point repeating the one before
/// it adds nothing (a pointer resting inside one cell samples it many times); the colour is the stroke's.
fn extended(mut stroke: PaintInputStroke, tick: &PaintInputStroke) -> PaintInputStroke {
    for point in &tick.points {
        if stroke.points.len() >= BITMAP_STROKE_MAXIMUM_POINTS {
            break;
        }
        if stroke.points.last() != Some(point) {
            stroke.points.push(*point);
        }
    }
    stroke
}

fn stroke_lands(_context: &BrushToolContext, event: Option<&brush_tool::Event>) -> bool {
    matches!(event, Some(brush_tool::Event::Stroke(request) | brush_tool::Event::Finish(request)) if request.lands(&request.stroke))
}

fn stream_carries_points(_context: &BrushToolContext, event: Option<&brush_tool::Event>) -> bool {
    matches!(event, Some(brush_tool::Event::Stream(request)) if !request.stroke.points.is_empty())
}

fn yield_stroke(_context: &mut BrushToolContext, event: Option<&brush_tool::Event>, sink: &mut Vec<Command<brush_tool::BrushTool>>) {
    let Some(brush_tool::Event::Stroke(request) | brush_tool::Event::Finish(request)) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(BITMAP_BRUSH_TOOL_KEY, BitmapMutation::PaintInputStroke(request.stroke.clone()))));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn begin_stream(context: &mut BrushToolContext, event: Option<&brush_tool::Event>, sink: &mut Vec<Command<brush_tool::BrushTool>>) {
    let Some(brush_tool::Event::Stream(request)) = event else { return };
    let stroke = extended(PaintInputStroke { points: Vec::new(), color: request.stroke.color }, &request.stroke);
    sink.push(Command::Effect(ToolYield::upsert(BITMAP_BRUSH_TOOL_KEY, BitmapMutation::PaintInputStroke(stroke.clone()))));
    context.stroke = Some(stroke);
}

fn continue_stream(context: &mut BrushToolContext, event: Option<&brush_tool::Event>, sink: &mut Vec<Command<brush_tool::BrushTool>>) {
    let (Some(brush_tool::Event::Stream(request)), Some(stroke)) = (event, context.stroke.take()) else { return };
    let stroke = extended(stroke, &request.stroke);
    sink.push(Command::Effect(ToolYield::upsert(BITMAP_BRUSH_TOOL_KEY, BitmapMutation::PaintInputStroke(stroke.clone()))));
    context.stroke = Some(stroke);
}

fn finish_stream(context: &mut BrushToolContext, event: Option<&brush_tool::Event>, sink: &mut Vec<Command<brush_tool::BrushTool>>) {
    let (Some(brush_tool::Event::Finish(request)), Some(stroke)) = (event, context.stroke.take()) else { return };
    let stroke = extended(stroke, &request.stroke);
    if request.lands(&stroke) {
        sink.push(Command::Effect(ToolYield::upsert(BITMAP_BRUSH_TOOL_KEY, BitmapMutation::PaintInputStroke(stroke))));
    } else {
        sink.push(Command::Effect(ToolYield::retract(BITMAP_BRUSH_TOOL_KEY)));
    }
    sink.push(Command::Effect(ToolYield::Commit));
}

fn cancel_stream(context: &mut BrushToolContext, _event: Option<&brush_tool::Event>, sink: &mut Vec<Command<brush_tool::BrushTool>>) {
    context.stroke = None;
    sink.push(Command::Effect(ToolYield::Abort));
}

machine::statechart! {
    machine brush_tool {
        context: BrushToolContext;
        event Event { Stroke(BrushToolRequest), Stream(BrushToolRequest), Finish(BrushToolRequest), Cancel }
        input: BrushToolContext;
        output: ();
        effect: ToolYield<BitmapMutation>;
        context_from_input: brush_tool_context;
        initial: idle;
        state idle {
            on Stroke if stroke_lands => idle do yield_stroke;
            on Finish if stroke_lands => idle do yield_stroke;
            on Stream if stream_carries_points => painting do begin_stream;
        }
        state painting {
            on Stream => painting do continue_stream;
            on Finish => idle do finish_stream;
            on Cancel => idle do cancel_stream;
        }
    }
}

/// 🧷️ The brush tool's host: its chart declares no timer, no invoke and no foreign effect.
pub struct BrushToolHost;

impl machine::Host<brush_tool::BrushTool> for BrushToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<BitmapMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// 🧭️ The brush chart on the shared gesture runner: a stroke is pinned to no revision (its leaf names cells and a
/// colour, so it repaints on any sample), every stroke is stamped `<appId>#paint-stroke`, and a resumed stroke gets its
/// accumulated leaf back from the ONE entry its open transaction holds.
impl GestureChart for brush_tool::BrushTool {
    type Tick = BrushToolRequest;
    type Host = BrushToolHost;

    const BASE_BOUND: bool = false;

    fn tool(_verb: &str) -> String {
        bitmap_brush_tool_id()
    }

    fn host() -> BrushToolHost {
        BrushToolHost
    }

    fn input() -> BrushToolContext {
        BrushToolContext::default()
    }

    fn restore(entries: &[(String, BitmapMutation)], _context: &semio_framework_value::DslValue) -> Option<BrushToolContext> {
        match entries {
            [(key, BitmapMutation::PaintInputStroke(stroke))] if key == BITMAP_BRUSH_TOOL_KEY => Some(BrushToolContext { stroke: Some(stroke.clone()) }),
            _ => None,
        }
    }

    fn event(phase: GesturePhase, _at_rest: bool, tick: Option<BrushToolRequest>) -> Option<brush_tool::Event> {
        match (phase, tick) {
            (GesturePhase::Once, Some(request)) => Some(brush_tool::Event::Stroke(request)),
            (GesturePhase::Stream, Some(request)) => Some(brush_tool::Event::Stream(request)),
            (GesturePhase::Commit, Some(request)) => Some(brush_tool::Event::Finish(request)),
            (GesturePhase::Abort(_), _) | (_, None) => None,
        }
    }
}

/// 🛠️ The brush as the window's streamed tool: driven through the dispatching window's gesture slot
/// (`GestureSlot::drive::<BitmapBrush>`), unmounted through `drive_chart_gesture::<brush_tool::BrushTool>`.
pub type BitmapBrush = ChartGesture<brush_tool::BrushTool>;

/// 🪪️ The `tool` every brush transaction is stamped with: `<appId>#paint-stroke`.
pub fn bitmap_brush_tool_id() -> String {
    format!("{}#{BITMAP_PAINT_STROKE_VERB}", crate::editor::bitmap::BITMAP_EDITOR_CONTROLLER_ID)
}
//#endregion 🛠️BrushTool

//#region 🖱️Pointer
/// 🖱️ One canvas pointer event on the input surface, in the WORLD coordinates both hosts stamp on it (`worldX` /
/// `worldY`, a move's `worldSamples`; one world unit is one sample cell), so the brush never needs the host camera.
#[derive(Clone, Debug, PartialEq)]
pub enum BitmapBrushPointer {
    Down { world: Option<[f64; 2]>, button: u32 },
    Move { samples: Vec<[f64; 2]> },
    Up { world: Option<[f64; 2]>, cancelled: bool },
}

/// 🧮️ What one pointer event asks of the brush: the phase and the cells it carries, and whether a stroke a lost
/// release left open is dropped first (`interrupt`, zero trace) because a fresh press opens a new one.
#[derive(Clone, Debug, PartialEq)]
pub struct BitmapBrushPointerStroke {
    pub phase: GesturePhase,
    pub points: Vec<BitmapStrokePoint>,
    pub interrupt: bool,
}

/// 📍️ The sample cell under `world`, clamped onto a `width × height` sample so a drag past the edge paints along it
/// instead of leaving the stroke; `None` for a non-finite sample or an empty bitmap.
fn pointer_cell(world: [f64; 2], width: u32, height: u32) -> Option<BitmapStrokePoint> {
    if width == 0 || height == 0 || !world.iter().all(|value| value.is_finite()) {
        return None;
    }
    let clamp = |value: f64, extent: u32| value.floor().clamp(0.0, f64::from(extent - 1)) as u32;
    Some(BitmapStrokePoint { x: clamp(world[0], width), y: clamp(world[1], height) })
}

/// 🖱️ Maps one pointer event onto the brush of a `width × height` sample whose window holds an `open` stroke or not:
/// a primary press opens a stroke (dropping one a lost release left open), a move while open streams its cells
/// (repeats collapsed), a release commits, a cancelled release aborts with `captureLost`. A hover move, a release
/// with nothing open and a non-primary press mean nothing (`None`).
pub fn bitmap_brush_pointer_stroke(pointer: &BitmapBrushPointer, width: u32, height: u32, open: bool) -> Option<BitmapBrushPointerStroke> {
    let cells = |samples: &[[f64; 2]]| {
        let mut cells: Vec<BitmapStrokePoint> = Vec::with_capacity(samples.len());
        for cell in samples.iter().filter_map(|sample| pointer_cell(*sample, width, height)) {
            if cells.last() != Some(&cell) {
                cells.push(cell);
            }
        }
        cells
    };
    match pointer {
        BitmapBrushPointer::Down { world, button: 0 } => {
            let points = cells(world.as_slice());
            (!points.is_empty()).then_some(BitmapBrushPointerStroke { phase: GesturePhase::Stream, points, interrupt: open })
        }
        BitmapBrushPointer::Down { .. } => None,
        BitmapBrushPointer::Move { samples } if open => {
            let points = cells(samples);
            (!points.is_empty()).then_some(BitmapBrushPointerStroke { phase: GesturePhase::Stream, points, interrupt: false })
        }
        BitmapBrushPointer::Move { .. } => None,
        BitmapBrushPointer::Up { cancelled: true, .. } if open => Some(BitmapBrushPointerStroke { phase: GesturePhase::Abort(ToolAbortReason::CaptureLost), points: Vec::new(), interrupt: false }),
        BitmapBrushPointer::Up { world, cancelled: false } if open => Some(BitmapBrushPointerStroke { phase: GesturePhase::Commit, points: cells(world.as_slice()), interrupt: false }),
        BitmapBrushPointer::Up { .. } => None,
    }
}
//#endregion 🖱️Pointer

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../../../../../🧪️tests/🧪️brush-tool/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
