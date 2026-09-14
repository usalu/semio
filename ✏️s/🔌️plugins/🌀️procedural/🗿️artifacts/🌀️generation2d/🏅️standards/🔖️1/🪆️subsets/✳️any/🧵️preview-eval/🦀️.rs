//! 🧵️ The ONE read-only `previewEval` tool run of the generation2d editor (`📋️tool-run-contract.md` §2,
//! §3.7; ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS), the twin of generation3d's `🧵️preview-eval`.
//!
//! ⏯️ The framework owns the lifecycle: the editor's `pending_effects` starts the run once an attached
//! preview window owes an evaluation, and start, pause, single step, abort and finalize are the
//! framework-reserved actions. [`PreviewEvalRunJob`] owns the scheduling: each algorithm unit is ONE
//! `flowEvalTick` hop addressed at a preview window, handed to the host through its [`ToolRunJobPort`],
//! after which the job waits until the hop settles. The hop advances the retained [`FlowEvalSession`]
//! its window kind evaluates ([`PreviewEvalTarget`]) and parks an [`semio_framework_plugin::ExtensionInvocation`]
//! for a contributed operator; `flowEvalResolve` folds the answer and wakes the job. Every observed node
//! transition becomes a trace record keyed by the node's entity id. Abort is host-driven by construction:
//! the job's close quiesces the sessions. No kernel release hop exists, because generation2d requests no
//! budgeted kernel job an extension could retain.
//!
//! 🪆️ Mounted at the subset level beside `✏️editor`, exactly where generation3d mounts its twin, so the
//! two artifacts' run modules stay one diff apart.

use semio_framework_job::{CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_os_flow::{flow_host_with_session, FlowEvalPublication, FlowEvalSession};
use semio_framework_plugin::{ArtifactInstanceOperationOwnerHandle, Effect, ExtensionInvocation, Fault, LocalizedLabel, ToolDefinition, ToolRunJobPort, ToolRunView, ViewModel};
use semio_framework_tool_run::{ToolRunCounter, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID, TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_START_ACTION_ID};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🪟️Addressing
/// ⏱️ One evaluation hop, naming the preview window whose target session it advances. An
/// `Effect::DispatchAction` carries no window of its own — the shell redispatches it under whichever
/// window is current — so the address rides on the payload.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {
    pub window_id: String,
    pub window_kind_id: String,
}

/// ✅️ One `evaluate` answer, echoed back onto the response action by `reactor::extension_response_args`
/// together with the window address the request carried.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-resolve")]
pub struct FlowEvalResolve {
    pub window_id: String,
    pub window_kind_id: String,
    pub node_hash: u64,
    pub output_json: String,
    #[value(default)]
    pub extension_id: String,
    #[value(default)]
    pub ok: bool,
    #[value(default)]
    pub fault_code: String,
    #[value(default)]
    pub fault_message: String,
}

/// 🪪️ The request id every hop effect of the run carries; the hop's own payload is its address.
const PREVIEW_EVAL_HOP_REQUEST: u64 = 103;

/// 🎯️ Which retained session a preview window's hop evaluates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreviewEvalTarget {
    /// 📄️ The document itself, into the session the flow and edit preview windows render.
    Document,
    /// 🧬️ The selected generation's patched fixture, into a session of its own.
    Generation,
}

impl PreviewEvalTarget {
    pub fn id(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::Generation => "generation",
        }
    }
}

/// 🪟️ One attached preview window: its instance id, its kind and the session it evaluates.
pub type PreviewEvalWindow<'a> = (&'a str, &'static str, PreviewEvalTarget);

/// ⏱️ The evaluation hop the run job hands the host, addressed to ONE preview window.
pub fn tick_effect(window_id: &str, window_kind_id: &str) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(PREVIEW_EVAL_HOP_REQUEST), action: "flowEvalTick".into(), args: Some(window_args(window_id, window_kind_id)), delay_ms: 0 }
}

/// 🚧️ Whether an evaluation of `fixture` may start or continue at all: an operator kind no contributed
/// extension serves faults identically on every hop, and only `setContributions` can change that.
pub fn may_rearm(fixture: &semio_framework_artifact_flow_flow::FlowFixture) -> bool {
    semio_framework_os_flow::unserved_flow_operator_kinds(fixture).is_empty()
}

/// 🪟️ The one argument object every hop carries, on the redispatch and on the extension request alike.
pub fn window_args(window_id: &str, window_kind_id: &str) -> dsl::DslValue {
    dsl::DslValue::object([
        ("windowId".to_string(), dsl::DslValue::String(window_id.to_string())),
        ("windowKindId".to_string(), dsl::DslValue::String(window_kind_id.to_string())),
    ])
}

/// 🪟️ Every attached preview window, in roster order, with the target its kind evaluates; `targets`
/// is the surface's own kind table.
pub fn attached_preview_windows<'a>(view: Option<&'a ViewModel>, targets: &[(&'static str, PreviewEvalTarget)]) -> Vec<PreviewEvalWindow<'a>> {
    view.map(|view| view.window_instances.iter().filter_map(|window| targets.iter().find(|(kind, _)| *kind == window.window_kind_id).map(|(kind, target)| (window.id.as_str(), *kind, *target))).collect()).unwrap_or_default()
}

/// 🧵️ Marks every attached preview window as owing an evaluation and wakes the live run job.
pub fn owe_attached_previews(sessions: PreviewEvalSessions<'_>, link: &mut PreviewEvalRunLink, windows: &[PreviewEvalWindow<'_>]) {
    let PreviewEvalSessions { document, generation } = sessions;
    for (window_id, _, target) in windows {
        match target {
            PreviewEvalTarget::Document => document.note_window_tick_outcome(window_id, true),
            PreviewEvalTarget::Generation => generation.note_window_tick_outcome(window_id, true),
        }
    }
    link.requested = None;
    link.wake();
}

/// 🩹️ What a LANDED GESTURE owes the attached previews, read off its own emit: artifact mutations moved
/// the document every evaluation reads, so each attached window owes one fresh evaluation; a gesture
/// that authored none owes a settled run nothing. Answers whether it owed.
pub fn owe_attached_previews_for_mutations<M, C, D>(sessions: PreviewEvalSessions<'_>, link: &mut PreviewEvalRunLink, windows: &[PreviewEvalWindow<'_>], servable: bool, emit: &mut semio_framework_plugin::Emit<M, C, D>) -> bool {
    if emit.artifact_mutations.is_empty() {
        link.wake();
        return false;
    }
    owe_attached_previews_carrying(sessions, link, windows, servable, emit);
    true
}

/// 🚦️ Owes every attached preview window an evaluation AND puts on `emit` whatever asks for it: a live,
/// unsettled run is woken through its port, a surface with no such run carries the run start itself —
/// the host polls `pending_effects` only on activity, so a debt nobody asks about is never paid.
pub fn owe_attached_previews_carrying<M, C, D>(sessions: PreviewEvalSessions<'_>, link: &mut PreviewEvalRunLink, windows: &[PreviewEvalWindow<'_>], servable: bool, emit: &mut semio_framework_plugin::Emit<M, C, D>) {
    owe_attached_previews(sessions, link, windows);
    let woken = link.port.is_some() && link.settled.is_none();
    if !windows.is_empty() && servable && !woken && link.requested.is_none() {
        link.requested = Some((PreviewEvalRunRequest::Start, None));
        emit.effects.push(run_action_effect(TOOL_RUN_START_ACTION_ID, dsl::DslValue::object([(TOOL_RUN_ARG_TOOL_ID.to_string(), dsl::DslValue::String(PREVIEW_EVAL_TOOL_ID.into()))])));
    }
}
//#endregion 🪟️Addressing

//#region ⏱️Tick
/// ⏱️ What one evaluation hop owes its caller: the extension work it parked and whether the target
/// session's evaluation text changed.
pub struct FlowEvalTickOutcome {
    pub extension_invocations: Vec<ExtensionInvocation>,
    pub publication: FlowEvalPublication,
}

/// 🏁️ Whether the hop that just ran leaves its window UNFINISHED: the evaluation said so, or it parked
/// answers it still waits for.
pub fn tick_is_unfinished(more: bool, parked_extension_invocations: usize) -> bool {
    more || parked_extension_invocations > 0
}

/// 🧮️ ONE evaluation hop over `fixture` into `session`, for the window it names. `retained_eval` is
/// the evaluation text the caller already published, so an unmoved evaluation republishes nothing.
pub fn evaluate_tick(window_id: &str, window_kind_id: &str, fixture: &semio_framework_artifact_flow_flow::FlowFixture, session: &mut FlowEvalSession, retained_eval: Option<&str>) -> FlowEvalTickOutcome {
    session.begin_window_tick(window_id);
    let mut host = flow_host_with_session(fixture, session);
    let more = session.tick(&mut host, None);
    let pending_extension_evals = host.take_pending_extension_evals();
    host.retire_cold();
    // 🌊️ ONE WAVE, ONE HOP — see the generation3d twin: every parked request had its inputs ready in
    // the same walk, so the whole level crosses to its plugin together.
    let extension_invocations: Vec<ExtensionInvocation> = pending_extension_evals
        .into_iter()
        .map(|pending| {
            let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
                ("operatorId".to_string(), dsl::DslValue::String(pending.operator_id.clone())),
                ("inputJson".to_string(), dsl::DslValue::String(pending.input_json.clone())),
                ("nodeHash".to_string(), dsl::DslValue::uint(pending.node_hash)),
                ("windowId".to_string(), dsl::DslValue::String(window_id.to_string())),
                ("windowKindId".to_string(), dsl::DslValue::String(window_kind_id.to_string())),
                ("extensionId".to_string(), dsl::DslValue::String(pending.extension_id.clone())),
            ]));
            ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve")
        })
        .collect();
    session.note_window_tick_outcome(window_id, tick_is_unfinished(more, extension_invocations.len()));
    if !extension_invocations.is_empty() {
        session.note_window_extensions_in_flight(window_id, extension_invocations.len());
    } else if more && !may_rearm(fixture) {
        session.abandon_window_tick(window_id);
    }
    FlowEvalTickOutcome { extension_invocations, publication: session.eval_publication_for(retained_eval) }
}

/// 🧬️ The hop a generate preview owes while no generation is selected: it ran, it has nothing to
/// evaluate, and it clears the evaluation it would otherwise keep showing.
pub fn settle_empty_tick(window_id: &str, session: &mut FlowEvalSession) {
    session.begin_window_tick(window_id);
    session.note_window_tick_outcome(window_id, false);
    if !session.eval_json().is_empty() {
        session.set_eval_json(String::new());
    }
}

/// ✅️ Folds one `evaluate` answer into `session` and settles the window's outstanding answer. An
/// answer that cannot fold (a faulted invocation, a cancelled envelope) gives the window up rather than
/// parking the identical request again.
pub fn resolve_eval(payload: &FlowEvalResolve, session: &mut FlowEvalSession) {
    let outcome = session.resolve_preview_eval(payload.node_hash, &payload.output_json);
    if payload.ok || payload.fault_code.is_empty() {
        session.clear_extension_evaluate_fault();
    } else {
        session.note_extension_evaluate_fault(semio_framework_os_flow::ExtensionEvaluateFault { extension_id: payload.extension_id.clone(), capability: "evaluate".to_string(), code: payload.fault_code.clone(), message: payload.fault_message.clone() });
    }
    let given_up = match &outcome {
        semio_framework_os_flow::PreviewEvalOutcome::Complete { output_json } => session.seed_node_cache(payload.node_hash, output_json).is_err(),
        semio_framework_os_flow::PreviewEvalOutcome::Cancelled => true,
        semio_framework_os_flow::PreviewEvalOutcome::Working => false,
    };
    if given_up {
        session.abandon_window_tick(&payload.window_id);
    }
    session.settle_window_extension(&payload.window_id);
}

/// 🔢️ A digest of the text a target evaluates, recorded by the hop that evaluated it and compared by
/// the next poll, so an undo, a redo or a remote edit owes the previews an evaluation no local gesture
/// announced.
pub fn preview_eval_digest(parts: &[&str]) -> u64 {
    parts.iter().fold(0xcbf2_9ce4_8422_2325, |hash, part| part.bytes().chain(std::iter::once(0x1f)).fold(hash, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)))
}
//#endregion ⏱️Tick

//#region ⏯️Run
#[path = "⏯️tool-run/🦀️.rs"]
mod tool_run;
pub use tool_run::*;
//#endregion ⏯️Run

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
