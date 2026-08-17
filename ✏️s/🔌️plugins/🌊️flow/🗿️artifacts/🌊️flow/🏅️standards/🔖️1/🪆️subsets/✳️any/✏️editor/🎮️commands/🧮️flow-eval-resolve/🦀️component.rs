//! 🧮️ 🧵️ Flow play app commands command — `flow-eval-resolve`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::host_from_snapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, Effect};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🧵️ The self-chaining action id of the off-main-thread evaluation loop — dispatched as a
/// `Effect` by `evaluate_result`/`flow_eval_tick::handle` and by `FlowPlayApp::pending_effects`.
pub const FLOW_EVAL_TICK_ACTION: &str = "flowEvalTick";

/// 🧵️ The `Effect` that arms/continues the off-main-thread `flowEvalTick` chain.
pub fn eval_tick_effect() -> Effect {
    Effect::DispatchAction {req: semio_framework_plugin::RequestId(106),  action: FLOW_EVAL_TICK_ACTION.into(), args: None, delay_ms: 0 }
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
