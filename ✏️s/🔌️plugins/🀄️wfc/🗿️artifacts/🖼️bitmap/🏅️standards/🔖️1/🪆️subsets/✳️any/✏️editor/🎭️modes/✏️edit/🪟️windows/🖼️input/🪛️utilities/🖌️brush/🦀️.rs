//! 🖌️ Input-window utility — Brush: paints the input sample with the window's armed palette colour. It is also the
//! brush TOOL: a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the `🛠️tool-machine` runner. A
//! stroke is ONE `ToolTransaction` holding ONE parametric `paint-input-stroke` leaf — the sampled cells in drawing
//! order and the colour — whether it arrives in one dispatch (`paint-stroke` without a phase) or streams over many
//! (`phase: stream`, then `commit`). Between dispatches the open transaction lives in the window transient, the
//! window paints committed ⊕ provisional, and every host abort leaves zero trace. Tool state is never history; the
//! committed leaf is (design §5, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
#![allow(unexpected_cfgs)]

use crate::editor::bitmap::modes::edit::windows::input::transient::{BitmapBrushToolState, BitmapInputWindowTransient};
use crate::mutations::{apply_bitmap_mutation, stroke_extent, BitmapMutation, BitmapStrokePoint, BITMAP_STROKE_MAXIMUM_POINTS};
use crate::schema::mutations::paint_input_stroke::PaintInputStroke;
use crate::BitmapSnapshot;
use machine::Command;
use semio_framework_plugin::{LocalizedLabel, UtilityCategory, UtilityDefinition};
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};

pub const UTILITY_ID: &str = "brush";

/// 🏷️ The verb every brush stroke dispatches, and so the `tool` half of its transactions: `<appId>#paint-stroke`.
pub const BITMAP_PAINT_STROKE_VERB: &str = "paint-stroke";

/// 🔑️ The transaction key of a stroke's leaf — every tick upserts it, so an open stroke holds ONE net leaf.
pub const BITMAP_BRUSH_TOOL_KEY: &str = "stroke:0";

/// 🧱️ Stitched into the input window kind by `super::super::definition`.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Brush", "Pinsel"), "paintbrush") }
}

//#region 🎚️Phase
/// 🎚️ Where one `paint-stroke` dispatch sits in a stroke: a one-shot `Once`, a `Stream` tick into the window's open
/// transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitmapStrokePhase {
    Once,
    Stream,
    Commit,
    Abort(ToolAbortReason),
}

impl BitmapStrokePhase {
    /// 🧩️ Reads `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's `reason` (`blur`,
    /// `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); `None` for an unknown one.
    pub fn parse(phase: Option<&str>, reason: Option<&str>) -> Option<Self> {
        match phase {
            None => Some(Self::Once),
            Some("stream") => Some(Self::Stream),
            Some("commit") => Some(Self::Commit),
            Some("abort") => reason.map_or(Some(ToolAbortReason::Tool), ToolAbortReason::parse).map(Self::Abort),
            Some(_) => None,
        }
    }

    /// 🔤️ The wire spelling of this phase (`None` for a one-shot).
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            Self::Once => None,
            Self::Stream => Some("stream"),
            Self::Commit => Some("commit"),
            Self::Abort(_) => Some("abort"),
        }
    }
}
//#endregion 🎚️Phase

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

/// ⏰️ The host clock a brush event runs on, so a transaction id minted at the first upsert is unique per admission
/// and per moment.
pub fn bitmap_brush_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🛠️ One window's brush tool for one dispatch: started at rest, or resumed from the stroke its window transient
/// persisted, driven by one event or one host abort, and persisted back while its transaction is open.
pub struct BitmapBrushTool {
    runner: ToolMachineRunner<brush_tool::BrushTool, BrushToolHost>,
    authoring_seed: String,
}

impl BitmapBrushTool {
    /// 🚀️ The tool at rest, for `<appId>#paint-stroke` under this admission's seed.
    pub fn start(authoring_seed: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(bitmap_brush_tool_id(), protocol::ActorId(authoring_seed.to_string()), BrushToolContext::default(), BrushToolHost)?;
        Ok(Self { runner, authoring_seed: authoring_seed.to_string() })
    }

    /// ⏯️ The stroke a window transient persisted, restored by stable ids with its open transaction; a state the
    /// current chart cannot restore is refused (`Closed`), so the caller drops it with zero trace.
    pub fn resume(state: &BitmapBrushToolState) -> Result<Self, ToolRefusal> {
        let definition = <brush_tool::BrushTool as machine::Machine>::definition();
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: definition.fingerprint, states: state.states.clone(), history: Vec::new(), done: false };
        let leaf: BitmapMutation = dsl::FromValue::from_value(state.stroke.clone()).map_err(|_| ToolRefusal::Closed)?;
        let BitmapMutation::PaintInputStroke(stroke) = leaf.clone() else { return Err(ToolRefusal::Closed) };
        let snapshot = machine::restore::<brush_tool::BrushTool, machine::NoMigrations>(&persisted, BrushToolContext { stroke: Some(stroke) }, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(state.transaction.clone(), vec![(BITMAP_BRUSH_TOOL_KEY.to_string(), leaf)]);
        let runner = ToolMachineRunner::resume(bitmap_brush_tool_id(), protocol::ActorId(state.authoring_seed.clone()), BrushToolContext::default(), snapshot, Some(transaction), BrushToolHost)?;
        Ok(Self { runner, authoring_seed: state.authoring_seed.clone() })
    }

    /// 🛋️ Whether the tool rests (no stroke in flight).
    pub fn at_rest(&self) -> bool {
        self.runner.at_rest()
    }

    /// 📨️ Runs one event on the host clock.
    pub fn send(&mut self, event: brush_tool::Event) -> Result<ToolStep<BitmapMutation>, ToolRefusal> {
        self.runner.send(event, bitmap_brush_tool_clock())
    }

    /// 🧯️ Host abort: the open transaction vanishes with zero trace and the tool rests.
    pub fn abort(&mut self, reason: ToolAbortReason) -> ToolStep<BitmapMutation> {
        self.runner.abort(reason)
    }

    /// 💾️ The state to persist in the window transient: `Some` only while a transaction is open.
    pub fn persist(self) -> Option<BitmapBrushToolState> {
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        let (_, leaf) = transaction.entries().iter().find(|(key, _)| key == BITMAP_BRUSH_TOOL_KEY)?;
        Some(BitmapBrushToolState { states: machine::persist(&snapshot).states, authoring_seed: self.authoring_seed, transaction: transaction.reference().clone(), stroke: dsl::ToValue::to_value(leaf) })
    }
}

/// 🪪️ The `tool` every brush transaction is stamped with: `<appId>#paint-stroke`.
pub fn bitmap_brush_tool_id() -> String {
    format!("{}#{BITMAP_PAINT_STROKE_VERB}", crate::editor::bitmap::BITMAP_EDITOR_CONTROLLER_ID)
}

/// 🧮️ What one `paint-stroke` dispatch does to the window's brush: its committed transaction (the stroke's ONE leaf
/// and its ref) when the dispatch ends a stroke that paints something, and the window transient to publish. A
/// one-shot interrupts an open stream (`captureLost`); a host abort, an unrestorable state, a stroke that paints no
/// cell and a cancel all leave zero trace.
pub fn bitmap_brush_dispatch(phase: BitmapStrokePhase, request: BrushToolRequest, transient: &BitmapInputWindowTransient, authoring_seed: &str) -> (Option<(protocol::TransactionRef, Vec<BitmapMutation>)>, BitmapInputWindowTransient) {
    let resumed = transient.brush.as_deref().and_then(|state| BitmapBrushTool::resume(state).ok());
    let rested = BitmapInputWindowTransient { brush: None };
    let mut tool = match (phase, resumed) {
        (BitmapStrokePhase::Abort(reason), Some(mut tool)) => {
            let _ = tool.abort(reason);
            return (None, rested);
        }
        (BitmapStrokePhase::Abort(_), None) => return (None, rested),
        (BitmapStrokePhase::Once, Some(mut tool)) => {
            let _ = tool.abort(ToolAbortReason::CaptureLost);
            tool
        }
        (_, Some(tool)) => tool,
        (_, None) => match BitmapBrushTool::start(authoring_seed) {
            Ok(tool) => tool,
            Err(_) => return (None, rested),
        },
    };
    let event = match phase {
        BitmapStrokePhase::Once => brush_tool::Event::Stroke(request),
        BitmapStrokePhase::Stream => brush_tool::Event::Stream(request),
        BitmapStrokePhase::Commit => brush_tool::Event::Finish(request),
        BitmapStrokePhase::Abort(_) => brush_tool::Event::Cancel,
    };
    match tool.send(event) {
        Ok(ToolStep::Committed(reference, mutations)) => (Some((reference, mutations)), rested),
        Ok(ToolStep::Open) => (None, BitmapInputWindowTransient { brush: tool.persist().map(Box::new) }),
        Ok(ToolStep::Idle | ToolStep::Aborted(..) | ToolStep::Empty(_)) | Err(_) => (None, rested),
    }
}

/// 👁️ The sample a window paints while its brush holds an open stroke: the provisional leaf applied to `document` —
/// a preview only this window sees, never history. `None` at rest or when the leaf paints nothing.
pub fn bitmap_brush_preview(document: &BitmapSnapshot, transient: &BitmapInputWindowTransient) -> Option<BitmapSnapshot> {
    let leaf: BitmapMutation = dsl::FromValue::from_value(transient.brush.as_ref()?.stroke.clone()).ok()?;
    let mut preview = document.clone();
    apply_bitmap_mutation(&mut preview, &leaf).ok()?;
    (preview != *document).then_some(preview)
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
    pub phase: BitmapStrokePhase,
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
            (!points.is_empty()).then_some(BitmapBrushPointerStroke { phase: BitmapStrokePhase::Stream, points, interrupt: open })
        }
        BitmapBrushPointer::Down { .. } => None,
        BitmapBrushPointer::Move { samples } if open => {
            let points = cells(samples);
            (!points.is_empty()).then_some(BitmapBrushPointerStroke { phase: BitmapStrokePhase::Stream, points, interrupt: false })
        }
        BitmapBrushPointer::Move { .. } => None,
        BitmapBrushPointer::Up { cancelled: true, .. } if open => Some(BitmapBrushPointerStroke { phase: BitmapStrokePhase::Abort(ToolAbortReason::CaptureLost), points: Vec::new(), interrupt: false }),
        BitmapBrushPointer::Up { world, cancelled: false } if open => Some(BitmapBrushPointerStroke { phase: BitmapStrokePhase::Commit, points: cells(world.as_slice()), interrupt: false }),
        BitmapBrushPointer::Up { .. } => None,
    }
}
//#endregion 🖱️Pointer

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../../../../../🧪️tests/🧪️brush-tool/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
