//! 🧮️ 🧵️ Flow play app commands command — `flow-eval-resolve`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::host_from_snapshot;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🧵️ The self-chaining action id of the off-main-thread evaluation loop — dispatched as a
/// `HostEffect` by `evaluate_result`/`flow_eval_tick::handle` and by `FlowPlayApp::pending_effects`.
pub const FLOW_EVAL_TICK_ACTION: &str = "flowEvalTick";

/// 🧵️ The `HostEffect` that arms/continues the off-main-thread `flowEvalTick` chain.
pub fn eval_tick_effect() -> HostEffect {
    HostEffect::DispatchAction { action: FLOW_EVAL_TICK_ACTION.into(), args: None, delay_ms: 0 }
}
//#endregion 🔖️Constants

//#region 🔖️Arm
/// 🧵️ Probes/arms the `flowEvalTick` chain via `FlowEvalSession::sync` — shared by `FlowCommand::Evaluate`,
/// the `auto-evaluate` extension effect, and `FlowPlayApp::pending_effects`.
pub fn evaluate_result(fixture: &FlowSnapshot, config: &FlowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, FlowConfigMutation> {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct FlowEvalResolve {
    pub node_hash: u64,
    pub output_json: String,
}

pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let _ = session.seed_node_cache(payload.node_hash, &payload.output_json);
    Ok(Emit { effects: vec![eval_tick_effect()], ..Default::default() })
}
