//! 🧭️ CAD edit-mode tool — Transform: the `🔄️machine` statechart (effects `ToolYield<CadMutation>`, driven by the
//! `🛠️tool-machine` runner) every object gesture commits through. A gumball translate / rotate / scale and the commit of
//! every `spatial.interaction` that lands objects (a construction, `transform.move`/`copy`/`rotate`/`scale*`, a mirror)
//! enter as [`CadToolEntry`]s and leave as ONE `ToolTransaction`: one parametric `drag-`/`rotate-`/`scale-selection` leaf
//! per touched pane plus the `create-object`s a construction or copy lands. Tool state is never history; the yielded
//! mutations are (design `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5).
#![allow(unexpected_cfgs)]

use crate::editor::cad::config::CadConfigMutation;
use crate::mutations::{drag_selection::DragSelection, rotate_selection::RotateSelection, scale_selection::ScaleSelection, CadMutation};
use crate::{CadPaneId, CadSnapshot};
use machine::Command;
use semio_framework_plugin::{ArtifactView, Emit};
use semio_framework_tool_machine::{ToolMachineRunner, ToolStep, ToolYield};
use std::sync::Arc;

/// 🪪️ The editor app id every transform-tool transaction's `tool` is scoped by: `<appId>#<verb>`.
pub const CAD_EDITOR_APP_ID: &str = "s.cad.cad@1/*#editor";

//#region 🎬️Record
/// 🎬️ How one transform moves its objects — the parameters of the leaf it yields.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CadTransformMotion {
    Drag { offset: [f64; 3] },
    Rotate { axis: [f64; 3], angle: f64 },
    Scale { factors: [f64; 3] },
}

/// 🎬️ One object transform the tool yields: the literal target ids, deduplicated in first-seen order, and the motion.
#[derive(Clone, Debug, PartialEq)]
pub struct CadTransformRecord {
    pub targets: Vec<String>,
    pub motion: CadTransformMotion,
}

impl CadTransformRecord {
    /// ✋️ A drag of `targets` by `offset` — the gumball translate, a `transform.move` commit, an inspector origin delta.
    pub fn drag(targets: impl IntoIterator<Item = String>, offset: [f64; 3]) -> Self {
        Self { targets: cad_unique_targets(targets), motion: CadTransformMotion::Drag { offset } }
    }

    /// 🔄️ A turn of `targets`, each in place, about `axis` by `angle` radians.
    pub fn rotate(targets: impl IntoIterator<Item = String>, axis: [f64; 3], angle: f64) -> Self {
        Self { targets: cad_unique_targets(targets), motion: CadTransformMotion::Rotate { axis, angle } }
    }

    /// 🔍️ A scaling of `targets`, each in place, by `factors`.
    pub fn scale(targets: impl IntoIterator<Item = String>, factors: [f64; 3]) -> Self {
        Self { targets: cad_unique_targets(targets), motion: CadTransformMotion::Scale { factors } }
    }

    /// 🎚️ Whether the motion is one a leaf admits and that moves: finite numbers, a non-zero axis, positive factors,
    /// and never the identity.
    pub fn moves(&self) -> bool {
        match self.motion {
            CadTransformMotion::Drag { offset } => offset.iter().all(|component| component.is_finite()) && offset != [0.0; 3],
            CadTransformMotion::Rotate { axis, angle } => axis.iter().all(|component| component.is_finite()) && axis != [0.0; 3] && angle.is_finite() && angle != 0.0,
            CadTransformMotion::Scale { factors } => factors.iter().all(|factor| factor.is_finite() && *factor > 0.0) && factors != [1.0; 3],
        }
    }

    /// 🧮️ The leaf this record yields for `pane` over `targets` — the pane's own share of the record.
    pub fn leaf(&self, pane: CadPaneId, targets: Vec<String>) -> CadMutation {
        match self.motion {
            CadTransformMotion::Drag { offset } => CadMutation::DragSelection(DragSelection { pane, targets, offset }),
            CadTransformMotion::Rotate { axis, angle } => CadMutation::RotateSelection(RotateSelection { pane, targets, axis, angle }),
            CadTransformMotion::Scale { factors } => CadMutation::ScaleSelection(ScaleSelection { pane, targets, factors }),
        }
    }

    /// 🗂️ One leaf per pane that materializes at least one target, in pane order, each over that pane's targets in
    /// record order — a target no pane owns is not the tool's to move.
    pub fn leaves(&self, base: &CadSnapshot) -> Vec<(CadPaneId, CadMutation)> {
        if !self.moves() {
            return Vec::new();
        }
        CadPaneId::all()
            .into_iter()
            .filter_map(|pane| {
                let scene = crate::cad_pane_local_scene(base, pane)?;
                let objects = crate::cad_scene_pane_objects(&scene, pane);
                let owned: Vec<String> = self.targets.iter().filter(|id| objects.iter().any(|object| &object.id == *id)).cloned().collect();
                (!owned.is_empty()).then(|| (pane, self.leaf(pane, owned)))
            })
            .collect()
    }
}

/// 🧱️ One entry of a transform-tool request: an object transform, or a leaf its caller already built (the
/// `create-object` a construction or a copy lands).
#[derive(Clone, Debug, PartialEq)]
pub enum CadToolEntry {
    Transform(CadTransformRecord),
    Leaf(CadMutation),
}

/// 🧹️ `targets` without repeats, in first-seen order — the one target list every leaf the tool yields carries.
pub fn cad_unique_targets(targets: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    targets.into_iter().filter(|id| seen.insert(id.clone())).collect()
}
//#endregion 🎬️Record

//#region 🛠️TransformTool
/// 📨️ What one transform-tool event carries: the committed document it yields against and the entries — dispatch
/// inputs, never tool state.
#[derive(Clone, Debug)]
pub struct CadToolRequest {
    pub base: Arc<CadSnapshot>,
    pub entries: Vec<CadToolEntry>,
}

/// 🧰️ The transform tool's context: empty, because every CAD object gesture reaches the guest as ONE record at its
/// release (the hosts preview the drag locally) — tool state, never history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CadTransformToolContext;

fn cad_transform_tool_context(input: CadTransformToolContext) -> CadTransformToolContext {
    input
}

fn entries_apply(_context: &CadTransformToolContext, event: Option<&cad_transform_tool::Event>) -> bool {
    matches!(event, Some(cad_transform_tool::Event::Records(request)) if !cad_tool_yields(&request.base, &request.entries).is_empty())
}

fn yield_entries(_context: &mut CadTransformToolContext, event: Option<&cad_transform_tool::Event>, sink: &mut Vec<Command<cad_transform_tool::CadTransformTool>>) {
    let Some(cad_transform_tool::Event::Records(request)) = event else { return };
    sink.extend(cad_tool_yields(&request.base, &request.entries).into_iter().map(|(key, mutation)| Command::Effect(ToolYield::upsert(key, mutation))));
    sink.push(Command::Effect(ToolYield::Commit));
}

machine::statechart! {
    machine cad_transform_tool {
        context: CadTransformToolContext;
        event Event { Records(CadToolRequest) }
        input: CadTransformToolContext;
        output: ();
        effect: ToolYield<CadMutation>;
        context_from_input: cad_transform_tool_context;
        initial: idle;
        state idle {
            on Records if entries_apply => idle do yield_entries;
        }
    }
}

/// 🧷️ The transform tool's host: its chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct CadTransformToolHost;

impl machine::Host<cad_transform_tool::CadTransformTool> for CadTransformToolHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<CadMutation>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: machine::TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: machine::TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        semio_framework_job::default_now_ms().unwrap_or(0)
    }
}

/// ⏰️ The host clock a transform-tool event runs on, so a transaction id minted at an upsert is unique per admission
/// AND per moment.
pub fn cad_transform_tool_clock() -> protocol::HybridLogicalTimestamp {
    protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 }
}

/// 🧮️ What the tool yields for `entries` on `base`, keyed and in order: each transform's leaf per touched pane, then
/// each prepared leaf — every one folded onto a running state, so a later entry sees the earlier ones. A leaf whose
/// outcome is a no-op, an Error or a Fatal on that state is not yielded: the tool never commits an edit it knows fails.
pub fn cad_tool_yields(base: &CadSnapshot, entries: &[CadToolEntry]) -> Vec<(String, CadMutation)> {
    let mut state = base.clone();
    let mut yields = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let leaves = match entry {
            CadToolEntry::Transform(record) => record.leaves(&state).into_iter().map(|(pane, leaf)| (format!("transform:{index}:{}", pane.model_definition_id()), leaf)).collect(),
            CadToolEntry::Leaf(leaf) => vec![(format!("leaf:{index}"), leaf.clone())],
        };
        for (key, leaf) in leaves {
            let outcome = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&leaf, &state);
            if outcome.messages().iter().any(|message| message.level >= protocol::Severity::Error || message.code.0 == "mutation.no-op") {
                continue;
            }
            let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), &state) else { continue };
            state = next;
            yields.push((key, leaf));
        }
    }
    yields
}

/// 🛠️ Runs `entries` through a transform tool at rest as ONE one-shot transaction — the ref minted from the admission's
/// `authoring_seed`, the host clock and `<appId>#<verb>`, and the yielded mutations in order. `None` when nothing moves
/// or lands: a stranger id, an identity motion or an empty request leaves zero trace.
pub fn cad_transform_tool_commit(verb: &str, authoring_seed: &str, base: &CadSnapshot, entries: Vec<CadToolEntry>) -> Option<(protocol::TransactionRef, Vec<CadMutation>)> {
    let mut runner = ToolMachineRunner::<cad_transform_tool::CadTransformTool, CadTransformToolHost>::start(format!("{CAD_EDITOR_APP_ID}#{verb}"), protocol::ActorId(authoring_seed.to_string()), CadTransformToolContext, CadTransformToolHost).ok()?;
    match runner.send(cad_transform_tool::Event::Records(CadToolRequest { base: Arc::new(base.clone()), entries }), cad_transform_tool_clock()).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}

/// 🧾️ The emit one committed transform-tool transaction publishes: ONE document edit stamped with its ref, labelled
/// from its leaves. A view without command authority (no authoring seed) publishes the yielded leaves plainly; nothing
/// yielded is the empty emit.
pub fn cad_transform_tool_emit(doc: &ArtifactView<'_, CadSnapshot>, verb: &str, entries: Vec<CadToolEntry>) -> Emit<CadMutation, CadConfigMutation> {
    let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
    match cad_transform_tool_commit(verb, authoring_seed, doc.snapshot, entries) {
        Some((transaction, mutations)) if !authoring_seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    }
}
//#endregion 🛠️TransformTool

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
