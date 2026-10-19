//! 🪟️ Window addressing of the preview evaluation: the flow-eval hop payloads, the hop effects and the owing of attached preview windows. See [`super`].

use super::*;

//#region 🪟️Addressing
/// 🪟️ The evaluation tick names the preview window that OWNS the evaluation it advances.
///
/// The tick publishes into one window's retained window transient, so the retained route is
/// window-scoped and its work refuses any command that does not name that window. Every tick is a
/// hop the run job hands the host as an `Effect::DispatchAction`, and an effect carries no window of
/// its own: the shell redispatches it under whichever window is current. Carrying the id ON THE
/// PAYLOAD is how `retained_window_transient_target` can capture the preview window's transient
/// authority (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {
    pub window_id: String,
    pub window_kind_id: String,
}

/// ✅️ One `evaluate` round trip's answer, echoed back onto the response action by
/// `reactor::extension_response_args` — including the window address, so the re-armed tick keeps
/// addressing the preview window that owns this evaluation.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "flow-eval-resolve")]
pub struct FlowEvalResolve {
    pub window_id: String,
    pub window_kind_id: String,
    pub node_hash: u64,
    pub output: semio_framework_value::DslValue,
    #[value(default)]
    pub fault: Option<semio_framework::Fault>,
    /// 🪪️ Which extension this answer came from, echoed back off the request's own correlation.
    #[value(default)]
    pub extension_id: String,
    /// ✅️ The SDK's own outcome flag (`reactor::extension_response_args`), present on every answer
    /// the runtime builds. Defaulted so a hand-authored fixture may still address only the fields it
    /// cares about.
    #[value(default)]
    pub ok: bool,
    #[value(default)]
    pub original_input:Option<semio_framework_value::DslValue>,
    pub input_valid:bool,
}

/// 🔺️ One budgeted `tessellate` round trip's answer, carrying the same echoed window address so a
/// resumable tessellation re-arms the tick on the window that owns the mesh it is building.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "flow-tessellate-resolve")]
pub struct FlowTessellateResolve {
    pub window_id: String,
    pub window_kind_id: String,
    pub node_hash: u64,
    pub handle: String,
    pub tolerance: f64,
    pub output: semio_framework_value::DslValue,
    #[value(default)]
    pub fault: Option<semio_framework::Fault>,
    #[value(default)]
    pub original_input:Option<semio_framework_value::DslValue>,
    pub input_valid:bool,
}

/// 🧯️ The geometry extension's `tessellateCancel` answer, echoed back onto the cancel's own response
/// action with the window address it was asked under. `retired` is how many kernel jobs the
/// extension actor actually retired — publication-free bookkeeping, kept because a cancel that
/// retires nothing on an actor that was supposed to be busy is the one symptom that distinguishes
/// "the gesture reached the kernel" from "the gesture reached only this process".
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "flow-tessellate-cancel-resolve")]
pub struct FlowTessellateCancelResolve {
    pub window_id: String,
    pub window_kind_id: String,
    #[value(default)]
    pub output:semio_framework_value::DslValue,
    #[value(default)]
    pub fault:Option<semio_framework::Fault>,
    #[value(default)]
    pub ok: bool,
    #[value(default)]
    pub original_input:Option<semio_framework_value::DslValue>,
    pub input_valid:bool,
}

/// 🧯️ The hop an aborted or restarted run hands the host to release the kernel work it left behind in
/// the geometry extension's own instance. Never a user gesture: the framework's `toolRunAbort` is the
/// gesture, and this hop is how its close reaches the registries the guest cannot see. Addressed at a
/// preview window because `reactor::extension_response_args` echoes the request's fields onto
/// `flowTessellateCancelResolve`, whose retained route needs a window it may name.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "flow-eval-release")]
pub struct FlowEvalRelease {
    #[value(default)]
    pub window_id: String,
    #[value(default)]
    pub window_kind_id: String,
}

/// 🪪️ The request id every hop effect of the run carries; the hop's own payload is its address.
pub(super) const PREVIEW_EVAL_HOP_REQUEST: u64 = 103;

/// ⏱️ The evaluation hop the run job hands the host, addressed to ONE preview window.
pub fn tick_effect(window_id: &str, window_kind_id: &str) -> Effect {
    hop_effect("flowEvalTick", window_id, window_kind_id)
}

/// 🧯️ The kernel release hop, addressed to ONE preview window.
pub fn release_effect(window_id: &str, window_kind_id: &str) -> Effect {
    hop_effect("flowEvalRelease", window_id, window_kind_id)
}

fn hop_effect(action: &str, window_id: &str, window_kind_id: &str) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(PREVIEW_EVAL_HOP_REQUEST), action: action.into(), args: Some(window_args(window_id, window_kind_id)), delay_ms: 0 }
}

/// 🚧️ Whether an unfinished evaluation of `fixture` may start or continue at all.
///
/// A graph whose operator kinds no contributed extension serves faults
/// `flow.extension-not-contributed`, and NOTHING a tick does can change that: the evaluator answers
/// `unknown kind`, the error dictionary is never cached, and the next tick recomputes the identical
/// miss. The pre-contribution chain therefore used to re-arm itself forever — and because every
/// settle drives a host `refreshUi`, the spin also starved the very push that would have fixed it
/// (45 s of live console with zero `[TRACE] contributions …` lines). The ONE thing that resumes
/// this chain is `setContributions`, whose route invalidates the retained session against the moved
/// registry generation and re-arms one tick per attached preview window
/// (`📓️contributions-rearm-2026-09-10.md`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🪪️ This supersedes `📓️fault-arm-symmetry-2026-09-10.md`'s "the fault arm is symmetric with the
/// ok arm": that report is about how a fault VALUE is encoded on the extension-result wire (pack,
/// not JSON) and says nothing about who owes the next tick. Symmetry of encoding is not symmetry of
/// continuation — a fault nothing in this process can clear owes no continuation at all. The run
/// therefore neither starts on such a graph nor continues it.
pub fn may_rearm(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> bool {
    semio_framework_os_flow::unserved_flow_operator_kinds(host_snapshot, &semio_framework_os_flow::flow_operator_registry()).is_empty()
}

/// 🪟️ The one argument object every hop of the chain carries, on the redispatch and on the
/// extension request alike — `reactor::extension_response_args` echoes an invocation request's own
/// fields back onto the response action, so the window address survives the round trip without the
/// SDK ever learning what it means.
pub fn window_args(window_id: &str, window_kind_id: &str) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([
        ("windowId".to_string(), semio_framework_value::DslValue::String(window_id.to_string())),
        ("windowKindId".to_string(), semio_framework_value::DslValue::String(window_kind_id.to_string())),
    ])
}

/// 🧾️ The APPLIED document-edit stack, folded into one number — what an evaluation is actually
/// computed from, independent of the route that changed it.
///
/// 🐛️ [`owe_attached_previews_for_mutations`] reads a gesture's OWN emit, so it can only ever see a
/// route the app itself handles. `undo` and `redo` are FRAMEWORK-RESERVED (`HISTORY_ACTION_IDS`,
/// intercepted before `dispatch_action` reaches the app), so an undo rewound the document and the app
/// never learned: measured on :6023, `mod+z` after a slider edit moved `height` 8 → 6 in the graph's
/// own published fixture and armed `canRedo`, while the preview kept delivering the 8-unit extrusion
/// and the console carried neither a `toolRunStart` nor a `flowEvalTick` after the chord
/// (`📓️react-final-sweep-2026-09-15.md` §7). The user pressed undo and the 3d view did not move.
///
/// Every route that changes the document moves this stack — a gesture's edit appends an applied entry,
/// an undo flips one to unapplied, a redo flips it back, `revertToCommand` and a checkpoint checkout
/// move several — so THIS is the rule, and the emit route stays the fast path a gesture takes without
/// waiting for a poll. Rows with no `edit_id` are cursor-motion and config entries, which the
/// evaluation does not read (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn applied_document_edits_digest(history: &semio_framework_plugin::HistoryView) -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut digest = OFFSET;
    for command in history.commands.iter().filter(|command| command.edit_id.is_some()) {
        digest = (digest ^ command.seq).wrapping_mul(PRIME);
        digest = (digest ^ u64::from(command.applied)).wrapping_mul(PRIME);
    }
    digest
}

/// 🧵️ Marks every attached preview window as owing an evaluation — what a gesture owes after it
/// changed what the evaluation would produce (`setActiveExample`, `setContributions`, the generation
/// commands, every viewer view command). The window's latch keeps the debt until a hop pays it, so a
/// window already chasing an answer is evaluated again the moment that answer settles; the live run
/// job is woken, and a surface without a live run starts one from its next `pending_effects`.
pub fn owe_attached_previews(session: &mut FlowEvalSession, link: &mut PreviewEvalRunLink, windows: &[(&str, &'static str)], grant: semio_framework_value::RetainedCloneGrant) -> Result<(), semio_framework_value::ValueError> {
    for (window_id, _) in windows {
        session.note_window_tick_outcome(window_id, true, grant)?;
    }
    link.requested = None;
    link.wake();
    Ok(())
}

/// 🩹️ What a LANDED GESTURE owes the attached previews, read off the gesture's own emit instead of a
/// roster of tool ids: a gesture that authored artifact mutations moved the document the evaluation is
/// computed from, so it owes every attached preview window exactly one fresh evaluation; one that
/// authored none changed nothing the evaluation reads and owes a settled run nothing. Answers whether
/// it owed.
///
/// 🪪️ A hand-kept roster can only approximate this, and did not: only `setActiveExample`, the six
/// generation commands, `setContributions` and a closing `importDocument` ever owed, so the inspector's
/// `patchFlowWidgets` — and `addWidget`, `removeWidget`, `nodeGraphEdit`, `deleteSelection`,
/// `reorganize` and every transform with it — moved the document and left the preview showing the old
/// geometry for good. `FlowEvalSession::abandon_window_tick` already names an EDIT as one of the three
/// gestures that may resume a chain; this is the editor half of that sentence
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, `📓️tick-arming-latch-2026-09-12.md`).
///
/// 🚦️ Recording the debt is only half of paying it. A LIVE run is woken through its port and picks the
/// debt up on its next step, but a surface whose run has already settled and been finalized holds
/// nothing that would ask the host for another `pending_effects` poll — the host polls once per
/// `refreshUi`, and it refreshes on ACTIVITY, so a document mutation that leaves the guest with nothing
/// further to say produces no poll at all. Measured: the inspector's slider recorded a perfect debt
/// (`owedAfter=["procedural-preview", "generation3d-generate-preview"]`) and the console then went
/// silent for 60 s with not one poll to read it. So a gesture with no live run carries the run start on
/// its OWN emit, under the same one-request latch `preview_eval_run_effects` asks through, and an
/// unservable graph carries nothing for the reason that function refuses one.
pub fn owe_attached_previews_for_mutations<M, C, D>(session: &mut FlowEvalSession, link: &mut PreviewEvalRunLink, windows: &[(&str, &'static str)], servable: bool, emit: &mut semio_framework_plugin::Emit<M, C, D>, grant: semio_framework_value::RetainedCloneGrant) -> Result<bool, semio_framework_value::ValueError> {
    if emit.artifact_mutations.is_empty() {
        link.wake();
        return Ok(false);
    }
    owe_attached_previews_carrying(session, link, windows, servable, emit, grant)?;
    Ok(true)
}

/// 🚦️ Owes every attached preview window an evaluation AND puts on `emit` whatever asks for it: a live
/// run is woken through its port and needs nothing, a surface with no live run carries the run start
/// itself. What every gesture route that owes the previews — a generation command, an example switch, a
/// contributions install, a closing import chunk, an inspector patch — hands back with its own emit.
///
/// 🪪️ Recording the debt alone is not enough, at ANY route: the host polls `pending_effects` once per
/// `refreshUi` and refreshes on ACTIVITY, so a gesture whose guest work leaves nothing further to say
/// produces no poll and the debt is never read. Measured on 6018 as `addGeneration` leaving the generate
/// preview empty for 120 s with not one `toolRunStart` in the console, and as an inspector slider that
/// moved `height` 6 → 7 against a preview that never re-evaluated
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
pub fn owe_attached_previews_carrying<M, C, D>(session: &mut FlowEvalSession, link: &mut PreviewEvalRunLink, windows: &[(&str, &'static str)], servable: bool, emit: &mut semio_framework_plugin::Emit<M, C, D>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(), semio_framework_value::ValueError> {
    owe_attached_previews(session, link, windows, grant)?;
    let woken = link.port.is_some() && link.settled.is_none();
    if !windows.is_empty() && servable && !woken && link.requested.is_none() {
        link.requested = Some((PreviewEvalRunRequest::Start, None));
        emit.effects.push(run_action_effect(TOOL_RUN_START_ACTION_ID, semio_framework_value::DslValue::object([(TOOL_RUN_ARG_TOOL_ID.to_string(), semio_framework_value::DslValue::String(PREVIEW_EVAL_TOOL_ID.into()))])));
    }
    Ok(())
}

/// 🪟️ The preview window kind this surface recognises, or `None` — the surface hands in its own
/// roster of preview kind ids (`procedural-preview` + `generation3d-generate-preview` for the
/// editor, `procedural-view-preview` for the viewer) and the answer is the `'static` id the window
/// transient owner is keyed by.
pub fn preview_kind(kind: &str, preview_kinds: &[&'static str]) -> Option<&'static str> {
    preview_kinds.iter().copied().find(|candidate| *candidate == kind)
}

/// 🪟️ Every attached preview window, in roster order — the windows a `flowEvalTick` chain can be
/// addressed to. Each holds its OWN retained evaluation publication, so each gets its own armed
/// chain; an empty answer means no surface is mounted yet and NOTHING may be armed at all.
pub fn attached_preview_windows<'a>(view: Option<&'a ViewModel>, preview_kinds: &[&'static str]) -> Vec<(&'a str, &'static str)> {
    view.map(|view| view.window_instances.iter().filter_map(|window| preview_kind(&window.window_kind_id, preview_kinds).map(|kind| (window.id.as_str(), kind))).collect()).unwrap_or_default()
}
//#endregion 🪟️Addressing
