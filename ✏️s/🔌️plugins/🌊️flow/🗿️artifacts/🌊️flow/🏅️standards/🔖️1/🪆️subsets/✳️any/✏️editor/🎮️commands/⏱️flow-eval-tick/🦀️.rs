//! 🧮️ 🧵️ Flow play app commands command — `flow-eval-tick`.

use crate::editor::flow::host_from_snapshot;
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, ExtensionInvocation, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants
/// 🧵️ The self-chaining action id of the off-main-thread evaluation loop — dispatched as a
/// `Effect` by `evaluate_result`/`flow_eval_tick::handle` and by `FlowPlayApp::pending_effects`.
pub const FLOW_EVAL_TICK_ACTION: &str = "flowEvalTick";

/// 🪪️ The request id every hop of the chain carries; the hop's own payload is its address, exactly
/// as in generation2d's `🧵️preview-eval` (`PREVIEW_EVAL_HOP_REQUEST`). Three call sites used to mint
/// three different ids (105/106/107) for one chain, which correlated nothing and only hid that the
/// hop was unaddressed.
const FLOW_EVAL_HOP_REQUEST: u64 = 107;

/// 🪟️ The one argument object every hop carries, on the redispatch and on the extension request
/// alike — `reactor::extension_response_args` echoes an invocation request's own fields onto the
/// response action, so `flowEvalResolve` finds the window its answer belongs to.
pub fn window_args(window_id: &str, window_kind_id: &str) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([
        ("windowId".to_string(), semio_framework_value::DslValue::String(window_id.to_string())),
        ("windowKindId".to_string(), semio_framework_value::DslValue::String(window_kind_id.to_string())),
    ])
}

/// 🧵️ The `Effect` that arms/continues the off-main-thread `flowEvalTick` chain, addressed at ONE
/// window. An `Effect::DispatchAction` carries no window of its own — the shell redispatches it
/// under whichever window is current — so the address rides on the payload.
pub fn eval_tick_effect(window_id: &str, window_kind_id: &str) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(FLOW_EVAL_HOP_REQUEST), action: FLOW_EVAL_TICK_ACTION.into(), args: Some(window_args(window_id, window_kind_id)), delay_ms: 0 }
}

/// 🚧️ Whether an evaluation of `host_snapshot` may start or continue at all: an operator kind no
/// contributed extension serves faults identically on every hop, so a chain that re-armed on it
/// would re-park the identical request forever. The twin of generation2d's `preview_eval::may_rearm`.
pub fn may_rearm(host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> bool {
    flow::unserved_flow_operator_kinds(host_snapshot, &flow::flow_operator_registry()).is_empty()
}
//#endregion 🔖️Constants

//#region 🔖️Arm
// 🧵️ The arm probe is owned by `commands::evaluate::evaluate_result`.
//#endregion 🔖️Arm

//#region 🔖️Evaluate
//#endregion 🔖️Evaluate

//#region 🔖️FlowEvalTick
//#endregion 🔖️FlowEvalTick

//#region 🔖️FlowEvalResolve
//#endregion 🔖️FlowEvalResolve

/// ⏱️ One evaluation hop, naming the window whose tick latch it discharges. The field-less payload
/// this replaces could not: every hop landed on whichever window was current, nothing discharged a
/// latch, and `FlowPlayApp::pending_effects` re-armed on the unchanged snapshot at the host's own
/// refresh cadence — 2015 `transient read registry is busy or exhausted` lines in ~3 s, measured on
/// :6016 (ticket 26/09/18 §5.3).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[derive(semio_framework_value::RetireOwned)]
pub struct FlowEvalTick {
    pub window_id: String,
    pub window_kind_id: String,
}

/// 🧮️ Advance `window_id`'s evaluation with the admitted window configuration, and owe exactly the
/// continuation that window still needs.
///
/// 🔒️ Three outcomes, one each: a hop that parked extension answers owes NO re-arm (the answers own
/// the continuation, and `settle_window_extension` hands it to the one that lands last); a hop with
/// unfinished work on a SERVABLE graph re-arms once, through the session's own latch, so a second
/// arming source cannot duplicate it; a hop with unfinished work that no installed extension can
/// serve abandons the window instead, because its next hop would park the identical request.
pub(crate) fn tick_result(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession, window_id: &str, window_kind_id: &str) -> Emit<FlowMutation, NoConfigMutation> {
    session.begin_window_tick(window_id, crate::editor::flow::cold_grant()).expect("flow cold session grant");
    let mut host = host_from_snapshot(snapshot, config, session);
    let more = session.tick_cold(&mut host, None);
    let parked = host.take_pending_extension_evals();
    let servable = may_rearm(&host.host_snapshot);
    let mut dependencies: std::collections::BTreeMap<_, _> = parked.iter().filter(|pending|!session.has_evaluation_progress(pending.node_hash)).map(|pending| (pending.neuron_id.clone(), flow::flow_inference_dependency_json(&host.host_snapshot, &pending.neuron_id))).collect();
    // 🧹️ The tick host owns a layout `OrderedMap` root that aborts the guest on a bare drop.
    host.retire_cold();
    let mut extension_invocations = Vec::new();
    // 🌊️ ONE WAVE, ONE HOP — every request the walk parked is independent of the others by
    // construction, so the whole dependency level crosses to its plugin on this one tick.
    for pending in parked {
        let resume=session.has_evaluation_progress(pending.node_hash);
        let hash=pending.node_hash;
        if session.invocation_origin_cold(window_id,hash).is_none(){
            let dependency_json=dependencies.remove(&pending.neuron_id).expect("initial invocation owns its original dependency source");
            session.retain_invocation_origin_cold(flow::host::FlowInvocationOriginLease::from_cold(flow::host::FlowInvocationOrigin{window_id:window_id.into(),window_kind_id:window_kind_id.into(),neuron_id:pending.neuron_id,extension_id:pending.extension_id,operator_id:pending.operator_id,node_hash:hash,input_json:pending.input_json,dependency_json,operator_version:format!("registry:{};geometry:1;policy:1",flow::flow_extension_registry_generation()),outer_node_hash:None}));
        }
        let origin=session.invocation_origin_cold(window_id,hash).expect("invocation source is retained before dispatch");
        extension_invocations.push(ExtensionInvocation::new(origin.extension_id.clone(),"evaluate",origin.request_json_cold(resume),"flowEvalResolve"));
    }
    session.note_window_tick_outcome(window_id, more || !extension_invocations.is_empty(), crate::editor::flow::cold_grant()).expect("flow cold session grant");
    let effects = if !extension_invocations.is_empty() {
        session.note_window_extensions_in_flight(window_id, extension_invocations.len(), crate::editor::flow::cold_grant()).expect("flow cold session grant");
        Vec::new()
    } else if more && servable && session.arm_window_tick(window_id, crate::editor::flow::cold_grant()).expect("flow cold session grant").0 {
        vec![eval_tick_effect(window_id, window_kind_id)]
    } else {
        if more && !servable {
            session.abandon_window_tick(window_id, crate::editor::flow::cold_grant()).expect("flow cold session grant");
        }
        Vec::new()
    };
    Emit { effects, extension_invocations, ..Default::default() }
}

pub fn handle(payload: &FlowEvalTick, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    if payload.window_id.is_empty() {
        return Err(Fault::from("flow-eval-tick-window-required"));
    }
    Ok(tick_result(&crate::flow_composed_snapshot(doc.snapshot, &doc.children)?, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, &payload.window_id, &payload.window_kind_id))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
