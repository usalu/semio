//! 🧮️ 🧵️ Flow play app commands command — `flow-eval-tick`.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::host_from_snapshot;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, ExtensionInvocation, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants
/// 🧵️ The self-chaining action id of the off-main-thread evaluation loop — dispatched as a
/// `Effect` by `evaluate_result`/`flow_eval_tick::handle` and by `FlowPlayApp::pending_effects`.
pub const FLOW_EVAL_TICK_ACTION: &str = "flowEvalTick";

/// 🧵️ The `Effect` that arms/continues the off-main-thread `flowEvalTick` chain.
pub fn eval_tick_effect() -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(107), action: FLOW_EVAL_TICK_ACTION.into(), args: None, delay_ms: 0 }
}
//#endregion 🔖️Constants

//#region 🔖️Arm
/// 🧵️ Probes/arms the `flowEvalTick` chain via `FlowEvalSession::sync` — shared by `FlowCommand::Evaluate`,
/// the `auto-evaluate` extension effect, and `FlowPlayApp::pending_effects`.
pub fn evaluate_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, NoConfigMutation> {
    let host = host_from_snapshot(fixture, config, session);
    if session.sync(&host) {
        Emit { effects: vec![eval_tick_effect()], ..Default::default() }
    } else {
        Emit::default()
    }
}
//#endregion 🔖️Arm

//#region 🔖️Evaluate
//#endregion 🔖️Evaluate

//#region 🔖️FlowEvalTick
//#endregion 🔖️FlowEvalTick

//#region 🔖️FlowEvalResolve
//#endregion 🔖️FlowEvalResolve

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct FlowEvalTick {}

/// 🧮️ Advance evaluation with the admitted window configuration.
pub(crate) fn tick_result(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, NoConfigMutation> {
    let mut host = host_from_snapshot(snapshot, config, session);
    let more = session.tick(&mut host);
    let effects = if more { vec![eval_tick_effect()] } else { Vec::new() };
    let mut extension_invocations = Vec::new();
    if let Some(pending) = host.take_pending_extension_eval() {
        let request_json = serde_json::json!({
            "operatorId": pending.operator_id,
            "inputJson": pending.input_json,
            "nodeHash": pending.node_hash,
        })
        .to_string();
        extension_invocations.push(ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve"));
    }
    Emit { effects, extension_invocations, ..Default::default() }
}

pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(tick_result(doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session))
}
