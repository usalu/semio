//! 🧮️ 🧵️ Flow play app commands command — `evaluate`.

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
pub struct Evaluate {}

pub fn handle(_payload: &Evaluate, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(evaluate_result(doc.snapshot, cfg.snapshot, session))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn evaluate_updates_preview_state_without_operations() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::Evaluate(Evaluate {}));
        assert!(result.mutations.is_empty(), "evaluate is a view action");
    }

    #[test]
    fn resolving_a_node_output_re_arms_the_tick_chain() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { node_hash: 42, output_json: "{}".into() }));
        assert!(result.mutations.is_empty(), "resolving is not a document edit");
    }

    #[test]
    fn flow_eval_session_neural_cache_is_per_instance_not_process_wide() {
        let a = FlowEvalSession::new();
        let b = FlowEvalSession::new();
        assert!(!std::sync::Arc::ptr_eq(&a.neural_cache(), &b.neural_cache()));
    }
}
//#endregion 🧪️Tests
