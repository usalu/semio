//! 🔄️ Blueprint utility — Transform: arms the world-space gumball on the selected frames.
//!
//! 🛠️ It is also the transform TOOL: a `🔄️machine` statechart whose effects are `ToolYield`s, driven by the
//! `🛠️tool-machine` runner. Every frame transform — a gumball drag, turn or scaling streamed by the canvas overlay
//! (`phase: stream | commit | abort`), and the one-shot `translateSelection`/`rotateSelection`/`scaleSelection` verbs —
//! enters as a [`LayoutFrameRecord`] and leaves as ONE `ToolTransaction` holding the parametric `drag-frames`,
//! `rotate-frames` or `scale-frames` leaf. A streamed gesture's open transaction lives in the Blueprint window transient
//! between dispatches and is previewed by that window only. Tool state is never history; the yielded leaf is (design
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5, §13).
#![allow(unexpected_cfgs)]

use crate::mutations::drag_frames::DragFrames;
use crate::mutations::rotate_frames::RotateFrames;
use crate::mutations::scale_frames::ScaleFrames;
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use machine::Command;
use semio_framework_plugin::{Emit, LocalizedLabel, NoConfigMutation, UtilityDefinition};
use semio_framework_tool_machine::{ToolAbortReason, ToolMachineRunner, ToolRefusal, ToolStep, ToolTransaction, ToolTransactionState, ToolYield};
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;

pub const UTILITY_ID: &str = "transform";

/// 🪪️ The editor app id every transform-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const LAYOUT_EDITOR_APP_ID: &str = "s.layout.layout@1/*#editor";

/// 🔑️ The transaction key of a gesture's parametric leaf — every record and stream tick upserts it, so an open
/// transaction always holds ONE net leaf.
pub const LAYOUT_TRANSFORM_TOOL_LEAF_KEY: &str = "frames:0";

/// 🎛️ Handle set the overlay draws. Every axis stays on; the host gumball reads the same flags
/// from the `meta:gumball` layer.
pub struct TransformUtilityOptions {
    pub move_axes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

/// 🧱️ Stitched into the blueprint window and the layout app manifest.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition { allows_actions_while_active: true, ..UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Transform", "Transformieren"), "move") }
}

/// 🎛️ Move, rotate, axis scale, and uniform scale are all armed.
pub fn options() -> TransformUtilityOptions {
    TransformUtilityOptions { move_axes: true, rotate: true, scale_axes: true, scale_uniform: true }
}

//#region 🎬️Record
/// 🎬️ How one frame transform moves its targets — the parameters of the leaf it yields.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutFrameMotion {
    Drag { dx: f64, dy: f64 },
    Rotate { pivot_x: f64, pivot_y: f64, angle: f64 },
    Scale { pivot_x: f64, pivot_y: f64, sx: f64, sy: f64 },
}

/// 🎬️ One frame transform the transform tool yields: the page, its literal target frames and the motion.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutFrameRecord {
    pub page_id: String,
    pub targets: Vec<String>,
    pub motion: LayoutFrameMotion,
}

impl LayoutFrameRecord {
    /// 🧮️ The parametric leaf this record yields.
    pub fn mutation(&self) -> LayoutMutation {
        let (page_id, targets) = (self.page_id.clone(), self.targets.clone());
        match self.motion {
            LayoutFrameMotion::Drag { dx, dy } => LayoutMutation::DragFrames(DragFrames { page_id, targets, dx, dy }),
            LayoutFrameMotion::Rotate { pivot_x, pivot_y, angle } => LayoutMutation::RotateFrames(RotateFrames { page_id, targets, pivot_x, pivot_y, angle }),
            LayoutFrameMotion::Scale { pivot_x, pivot_y, sx, sy } => LayoutMutation::ScaleFrames(ScaleFrames { page_id, targets, pivot_x, pivot_y, sx, sy }),
        }
    }

    /// 🧬️ The record a parametric leaf states — how a resumed stream recovers its accumulated transform.
    pub fn from_leaf(leaf: &LayoutMutation) -> Option<Self> {
        let (page_id, targets, motion) = match leaf {
            LayoutMutation::DragFrames(leaf) => (&leaf.page_id, &leaf.targets, LayoutFrameMotion::Drag { dx: leaf.dx, dy: leaf.dy }),
            LayoutMutation::RotateFrames(leaf) => (&leaf.page_id, &leaf.targets, LayoutFrameMotion::Rotate { pivot_x: leaf.pivot_x, pivot_y: leaf.pivot_y, angle: leaf.angle }),
            LayoutMutation::ScaleFrames(leaf) => (&leaf.page_id, &leaf.targets, LayoutFrameMotion::Scale { pivot_x: leaf.pivot_x, pivot_y: leaf.pivot_y, sx: leaf.sx, sy: leaf.sy }),
            _ => return None,
        };
        Some(Self { page_id: page_id.clone(), targets: targets.clone(), motion })
    }

    /// 🎚️ Whether the motion is one a leaf admits and that moves: finite numbers, positive factors, not the identity.
    pub fn moves(&self) -> bool {
        match self.motion {
            LayoutFrameMotion::Drag { dx, dy } => dx.is_finite() && dy.is_finite() && (dx, dy) != (0.0, 0.0),
            LayoutFrameMotion::Rotate { pivot_x, pivot_y, angle } => pivot_x.is_finite() && pivot_y.is_finite() && angle.is_finite() && angle != 0.0,
            LayoutFrameMotion::Scale { pivot_x, pivot_y, sx, sy } => pivot_x.is_finite() && pivot_y.is_finite() && sx.is_finite() && sy.is_finite() && sx > 0.0 && sy > 0.0 && (sx, sy) != (1.0, 1.0),
        }
    }

    /// 🔎️ Whether this record moves anything on `base`: an admissible non-identity motion and at least one target that
    /// is an unlocked frame of its page.
    pub fn applies_to(&self, base: &LayoutSnapshot) -> bool {
        self.moves() && self.target_frames(base).any(|(page, frame)| !crate::frame_edits_blocked(base, page, frame))
    }

    /// 🔒️ The tool-level refusal: the motion moves, yet every target that is a frame of the page is locked. A record with
    /// one movable target is yielded instead, and its leaf reports the locked rest as `mutation.partial`.
    pub fn refused_as_locked(&self, base: &LayoutSnapshot) -> bool {
        self.moves() && !self.applies_to(base) && self.target_frames(base).next().is_some()
    }

    fn target_frames<'a>(&'a self, base: &'a LayoutSnapshot) -> impl Iterator<Item = (&'a crate::Page, &'a crate::Frame)> + 'a {
        base.pages.iter().filter(|page| page.id == self.page_id).flat_map(move |page| page.frames.iter().filter(|frame| self.targets.iter().any(|id| id == frame.id())).map(move |frame| (page, frame)))
    }

    /// ➕️ This transform followed by `tick` as ONE net transform: offsets add, angles add, factors multiply — only for the
    /// same page and targets, the same kind and (for a turn or scaling) the same pivot; `None` otherwise.
    pub fn then(&self, tick: &Self) -> Option<Self> {
        if (&self.page_id, &self.targets) != (&tick.page_id, &tick.targets) {
            return None;
        }
        let motion = match (self.motion, tick.motion) {
            (LayoutFrameMotion::Drag { dx, dy }, LayoutFrameMotion::Drag { dx: next_dx, dy: next_dy }) => LayoutFrameMotion::Drag { dx: dx + next_dx, dy: dy + next_dy },
            (LayoutFrameMotion::Rotate { pivot_x, pivot_y, angle }, LayoutFrameMotion::Rotate { pivot_x: next_x, pivot_y: next_y, angle: next }) if (pivot_x, pivot_y) == (next_x, next_y) => LayoutFrameMotion::Rotate { pivot_x, pivot_y, angle: angle + next },
            (LayoutFrameMotion::Scale { pivot_x, pivot_y, sx, sy }, LayoutFrameMotion::Scale { pivot_x: next_x, pivot_y: next_y, sx: next_sx, sy: next_sy }) if (pivot_x, pivot_y) == (next_x, next_y) => LayoutFrameMotion::Scale { pivot_x, pivot_y, sx: sx * next_sx, sy: sy * next_sy },
            _ => return None,
        };
        Some(Self { page_id: self.page_id.clone(), targets: self.targets.clone(), motion })
    }
}

/// 🧹️ `targets` without repeats, in first-seen order — the one target list every leaf the transform tool yields carries.
pub fn layout_unique_targets(targets: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    targets.into_iter().filter(|id| seen.insert(id.clone())).collect()
}
//#endregion 🎬️Record

//#region 🛠️TransformTool
/// 📨️ What one transform-tool event carries: the committed document it yields against and this dispatch's record (the
/// tail of a commit may be absent) — dispatch inputs, never tool state.
#[derive(Clone, Debug)]
pub struct TransformToolRequest {
    pub base: Arc<LayoutSnapshot>,
    pub record: Option<LayoutFrameRecord>,
}

/// 🧰️ The transform tool's context: the one transform a streamed gesture has accumulated so far (`None` at rest).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransformToolContext {
    pub stream: Option<LayoutFrameRecord>,
}

fn transform_tool_context(input: TransformToolContext) -> TransformToolContext {
    input
}

fn request_applies(request: &TransformToolRequest) -> bool {
    request.record.as_ref().is_some_and(|record| record.applies_to(&request.base))
}

fn once_applies(_context: &TransformToolContext, event: Option<&transform_tool::Event>) -> bool {
    matches!(event, Some(transform_tool::Event::Once(request)) if request_applies(request))
}

fn stream_applies(_context: &TransformToolContext, event: Option<&transform_tool::Event>) -> bool {
    matches!(event, Some(transform_tool::Event::Stream(request)) if request_applies(request))
}

fn yield_once(_context: &mut TransformToolContext, event: Option<&transform_tool::Event>, sink: &mut Vec<Command<transform_tool::TransformTool>>) {
    let Some(transform_tool::Event::Once(TransformToolRequest { record: Some(record), .. })) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(LAYOUT_TRANSFORM_TOOL_LEAF_KEY, record.mutation())));
    sink.push(Command::Effect(ToolYield::Commit));
}

fn begin_stream(context: &mut TransformToolContext, event: Option<&transform_tool::Event>, sink: &mut Vec<Command<transform_tool::TransformTool>>) {
    let Some(transform_tool::Event::Stream(TransformToolRequest { record: Some(record), .. })) = event else { return };
    context.stream = Some(record.clone());
    sink.push(Command::Effect(ToolYield::upsert(LAYOUT_TRANSFORM_TOOL_LEAF_KEY, record.mutation())));
}

fn continue_stream(context: &mut TransformToolContext, event: Option<&transform_tool::Event>, sink: &mut Vec<Command<transform_tool::TransformTool>>) {
    let Some(transform_tool::Event::Stream(request)) = event else { return };
    let Some(stream) = context.stream.take() else { return };
    let stream = request.record.as_ref().and_then(|tick| stream.then(tick)).unwrap_or(stream);
    sink.push(Command::Effect(ToolYield::upsert(LAYOUT_TRANSFORM_TOOL_LEAF_KEY, stream.mutation())));
    context.stream = Some(stream);
}

fn finish_stream(context: &mut TransformToolContext, event: Option<&transform_tool::Event>, sink: &mut Vec<Command<transform_tool::TransformTool>>) {
    let Some(transform_tool::Event::Finish(request)) = event else { return };
    let Some(stream) = context.stream.take() else { return };
    let stream = request.record.as_ref().and_then(|tail| stream.then(tail)).unwrap_or(stream);
    sink.push(Command::Effect(if stream.applies_to(&request.base) { ToolYield::upsert(LAYOUT_TRANSFORM_TOOL_LEAF_KEY, stream.mutation()) } else { ToolYield::retract(LAYOUT_TRANSFORM_TOOL_LEAF_KEY) }));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine transform_tool {
        context: TransformToolContext;
        event Event { Once(TransformToolRequest), Stream(TransformToolRequest), Finish(TransformToolRequest) }
        input: TransformToolContext;
        output: ();
        effect: ToolYield<LayoutMutation>;
        context_from_input: transform_tool_context;
        initial: idle;
        state idle {
            on Once if once_applies => idle do yield_once;
            on Stream if stream_applies => streaming do begin_stream;
        }
        state streaming {
            on Stream => streaming do continue_stream;
            on Finish => idle do finish_stream;
        }
    }
}

/// 🎚️ Where one dispatch of a frame transform sits in a transform-tool gesture: a one-shot `Once`, a `Stream` tick into
/// the window's open transaction, the `Commit` that ends it, or a host `Abort` with its reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutTransformPhase {
    Once,
    Stream,
    Commit,
    Abort(ToolAbortReason),
}

impl LayoutTransformPhase {
    /// 🧩️ Reads a transform verb's `phase` (`stream` | `commit` | `abort`, absent = one-shot) and an abort's `reason`
    /// (`blur`, `captureLost`, `baseMoved`, `frozen`, `retired`; absent = `tool`); `None` for an unknown one.
    pub fn parse(phase: Option<&str>, reason: Option<&str>) -> Option<Self> {
        match phase {
            None => Some(Self::Once),
            Some("stream") => Some(Self::Stream),
            Some("commit") => Some(Self::Commit),
            Some("abort") => reason.map_or(Some(ToolAbortReason::Tool), ToolAbortReason::parse).map(Self::Abort),
            Some(_) => None,
        }
    }
}

/// 🧷️ The transform tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct TransformToolHost;

impl machine::Host<transform_tool::TransformTool> for TransformToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<LayoutMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// ⏰️ The host clock a transform-tool event runs on: the host's wall time, so a transaction id minted at an upsert is
/// unique per admission AND per moment.
pub fn layout_transform_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🧷️ One provisional entry of a persisted transform-tool transaction, its mutation in value form so the window
/// transient retires it like any other value.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct LayoutTransformToolEntry {
    pub key: String,
    pub mutation: dsl::DslValue,
}

/// 💾️ A Blueprint window's in-flight transform-tool gesture, persisted in its window transient between dispatches —
/// ephemeral local tool state, never history: the statechart configuration by stable ids, the verb, the admission and
/// document revision it opened on, and the open transaction's provisional entries.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct LayoutTransformToolState {
    pub states: Vec<String>,
    pub verb: String,
    pub authoring_seed: String,
    pub base_revision: String,
    pub transaction: protocol::TransactionRef,
    pub entries: Vec<LayoutTransformToolEntry>,
}

store::artifact_retire_struct!(LayoutTransformToolEntry { key, mutation });
store::artifact_retire_struct!(LayoutTransformToolState { states, verb, authoring_seed, base_revision, transaction, entries });

/// 🛠️ One window's transform tool for one dispatch: started at rest, or resumed from the gesture its window transient
/// persisted, driven by one event or one host abort, and persisted back while its transaction is open.
pub struct LayoutTransformTool {
    runner: ToolMachineRunner<transform_tool::TransformTool, TransformToolHost>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
}

impl LayoutTransformTool {
    /// 🚀️ The tool at rest, for `<appId>#<verb>` under this admission's seed and document revision.
    pub fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(format!("{LAYOUT_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), TransformToolContext::default(), TransformToolHost)?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    /// ⏯️ The gesture a window transient persisted, restored by stable ids with its open transaction; a state the current
    /// chart cannot restore is refused (`Closed`), so the caller drops it with zero trace.
    pub fn resume(state: &LayoutTransformToolState) -> Result<Self, ToolRefusal> {
        let definition = <transform_tool::TransformTool as machine::Machine>::definition();
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: definition.fingerprint, states: state.states.clone(), history: Vec::new(), done: false };
        let entries = state.entries.iter().map(|entry| Ok((entry.key.clone(), dsl::FromValue::from_value(entry.mutation.clone()).map_err(|_| ToolRefusal::Closed)?))).collect::<Result<Vec<(String, LayoutMutation)>, ToolRefusal>>()?;
        let stream = entries.iter().find(|(key, _)| key == LAYOUT_TRANSFORM_TOOL_LEAF_KEY).and_then(|(_, leaf)| LayoutFrameRecord::from_leaf(leaf));
        let snapshot = machine::restore::<transform_tool::TransformTool, machine::NoMigrations>(&persisted, TransformToolContext { stream }, &[]).map_err(|_| ToolRefusal::Closed)?;
        let runner = ToolMachineRunner::resume(format!("{LAYOUT_EDITOR_APP_ID}#{}", state.verb), protocol::ActorId(state.authoring_seed.clone()), TransformToolContext::default(), snapshot, Some(ToolTransaction::resume(state.transaction.clone(), entries)), TransformToolHost)?;
        Ok(Self { runner, verb: state.verb.clone(), authoring_seed: state.authoring_seed.clone(), base_revision: state.base_revision.clone() })
    }

    /// 🛋️ Whether the tool rests (no gesture in flight).
    pub fn at_rest(&self) -> bool {
        self.runner.at_rest()
    }

    /// 📨️ Runs one event on the host clock.
    pub fn send(&mut self, event: transform_tool::Event) -> Result<ToolStep<LayoutMutation>, ToolRefusal> {
        self.runner.send(event, layout_transform_tool_clock())
    }

    /// 🧯️ Host abort: the open transaction vanishes with zero trace and the tool rests.
    pub fn abort(&mut self, reason: ToolAbortReason) -> ToolStep<LayoutMutation> {
        self.runner.abort(reason)
    }

    /// 💾️ The state to persist in the window transient: `Some` only while a transaction is open.
    pub fn persist(self) -> Option<LayoutTransformToolState> {
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(LayoutTransformToolState {
            states: machine::persist(&snapshot).states,
            verb: self.verb,
            authoring_seed: self.authoring_seed,
            base_revision: self.base_revision,
            transaction: transaction.reference().clone(),
            entries: transaction.entries().iter().map(|(key, mutation)| LayoutTransformToolEntry { key: key.clone(), mutation: dsl::ToValue::to_value(mutation) }).collect(),
        })
    }
}

/// 📤️ What one transform dispatch publishes: the emit (ONE committed transaction, or nothing) and — when the window's
/// persisted gesture changed — its new transform-tool state (`Some(None)` clears it).
pub struct LayoutTransformDispatch {
    pub emit: Emit<LayoutMutation, NoConfigMutation>,
    pub tool: Option<Option<Box<LayoutTransformToolState>>>,
}

/// 🛠️ Drives a window's transform tool through ONE dispatch of a frame transform. `Once` commits the record as one
/// transaction; `Stream` upserts it into the window's open transaction — opening it on the first tick — which the window
/// transient persists and the window previews; `Commit` folds the tail in and commits the whole gesture as ONE edit;
/// `Abort` drops the open gesture with zero trace. A commit that finds no open gesture publishes nothing. An open gesture
/// another verb (or a one-shot) interrupts is aborted `captureLost`; one whose document moved under it is aborted
/// `baseMoved`, and a stream tick or commit that found it is dropped with it. `notice` is the one localized sentence a
/// request whose every target is locked raises.
pub fn layout_transform_dispatch(verb: &str, phase: LayoutTransformPhase, record: Option<LayoutFrameRecord>, base: &LayoutSnapshot, open: Option<&LayoutTransformToolState>, authoring_seed: &str, base_revision: &str, notice: Option<&str>) -> LayoutTransformDispatch {
    let quiet = |tool: Option<Option<Box<LayoutTransformToolState>>>| LayoutTransformDispatch { emit: Emit { ui_scope: semio_framework::kernel::UiDirtyScope::None, ..Emit::default() }, tool };
    let cleared = open.is_some().then_some(None);
    let resumed = open.and_then(|state| LayoutTransformTool::resume(state).ok());
    let resumed = match (resumed, phase) {
        (Some(mut tool), LayoutTransformPhase::Abort(reason)) => {
            tool.abort(reason);
            return LayoutTransformDispatch { emit: Emit::default(), tool: cleared };
        }
        (None, LayoutTransformPhase::Abort(_)) => return quiet(cleared),
        (Some(mut tool), _) if tool.base_revision != base_revision => {
            tool.abort(ToolAbortReason::BaseMoved);
            if phase != LayoutTransformPhase::Once {
                return LayoutTransformDispatch { emit: Emit::default(), tool: cleared };
            }
            None
        }
        (Some(mut tool), _) if tool.verb != verb || phase == LayoutTransformPhase::Once => {
            tool.abort(ToolAbortReason::CaptureLost);
            None
        }
        (resumed, _) => resumed,
    };
    if resumed.is_none() && phase == LayoutTransformPhase::Commit {
        return quiet(cleared);
    }
    let Some(mut tool) = resumed.or_else(|| LayoutTransformTool::start(verb, authoring_seed, base_revision).ok()) else { return quiet(cleared) };
    let refused = record.as_ref().is_some_and(|record| record.refused_as_locked(base));
    let request = TransformToolRequest { base: Arc::new(base.clone()), record };
    let event = match phase {
        LayoutTransformPhase::Stream => transform_tool::Event::Stream(request),
        LayoutTransformPhase::Commit => transform_tool::Event::Finish(request),
        LayoutTransformPhase::Once | LayoutTransformPhase::Abort(_) => transform_tool::Event::Once(request),
    };
    let mut emit = match tool.send(event) {
        Ok(ToolStep::Committed(transaction, mutations)) if !authoring_seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Ok(ToolStep::Committed(_, mutations)) => Emit::mutations(mutations),
        Ok(ToolStep::Idle) if refused => notice.map_or_else(Emit::default, |notice| Emit::effect(semio_framework::kernel::Effect::Notify { message: notice.to_string() })),
        Ok(_) | Err(_) => Emit::default(),
    };
    let persisted = tool.persist();
    if emit.artifact_mutations.is_empty() && persisted.is_none() && open.is_none() && !refused {
        emit.ui_scope = semio_framework::kernel::UiDirtyScope::None;
    }
    let tool = (open.is_some() || persisted.is_some()).then(|| persisted.map(Box::new));
    LayoutTransformDispatch { emit, tool }
}

/// 👁️ The document a window paints while its transform tool holds an open transaction: the provisional entries applied
/// to `document` — a preview only this window sees, never history.
pub fn layout_transform_tool_preview(document: &LayoutSnapshot, state: &LayoutTransformToolState) -> LayoutSnapshot {
    use protocol::{Mutation, MutationDiff};
    state.entries.iter().filter_map(|entry| dsl::FromValue::from_value(entry.mutation.clone()).ok()).fold(document.clone(), |preview, mutation: LayoutMutation| mutation.diff(&preview).diff().apply(&preview).unwrap_or(preview))
}
//#endregion 🛠️TransformTool

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
