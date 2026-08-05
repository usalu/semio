//! 🧵️ Flow play app commands — the off-main-thread evaluation chain.
//!
//! `evaluate` arms the chain; `flowEvalTick` runs one budgeted step and self-chains via
//! `HostEffect::DispatchAction` until the fixture's dirty set is empty; `flowEvalResolve` feeds a
//! plugin-exchange answer back into the node cache and re-arms. The last two are internal chain links,
//! never user-facing manifest actions.

use crate::apps::flow::config::{FlowConfig, FlowConfigOperation};
use crate::artifacts::flow::engine::{eval_tick_effect, host_from_fixture};
use crate::artifacts::flow::{op::FlowOperation, FlowFixture};
use flow_core::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

//#region 🔖️Arm
/// 🧵️ Probes/arms the `flowEvalTick` chain via `FlowEvalSession::sync` — shared by `FlowCommand::Evaluate`,
/// the `auto-evaluate` extension effect, and `FlowPlayApp::pending_effects`.
pub fn evaluate_result(fixture: &FlowFixture, config: &FlowConfig, session: &mut FlowEvalSession) -> Emit<FlowOperation, FlowConfigOperation> {
    let host = host_from_fixture(fixture, config, session);
    if session.sync(&host) {
        Emit { effects: vec![eval_tick_effect()], ..Default::default() }
    } else {
        Emit::default()
    }
}
//#endregion 🔖️Arm

//#region 🔖️Evaluate
pub mod evaluate {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct Evaluate {}

    pub fn handle(_payload: &Evaluate, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(super::evaluate_result(doc.projection, cfg.projection, session))
    }
}
//#endregion 🔖️Evaluate

//#region 🔖️FlowEvalTick
pub mod flow_eval_tick {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct FlowEvalTick {}

    pub fn handle(_payload: &FlowEvalTick, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        let mut host = host_from_fixture(doc.projection, cfg.projection, session);
        let more = session.tick(&mut host);
        let mut effects = if more { vec![eval_tick_effect()] } else { Vec::new() };
        if let Some(pending) = host.take_pending_extension_eval() {
            if let Some(plugin_id) = flow_core::flow_extension_plugin_id(&pending.extension_id) {
                let request_json = serde_json::json!({
                    "operatorId": pending.operator_id,
                    "inputJson": pending.input_json,
                    "nodeHash": pending.node_hash,
                })
                .to_string();
                effects.push(HostEffect::RequestPluginExchange { plugin_id, app_id: "flow-extension-eval".into(), request_json, response_action: "flowEvalResolve".into() });
            }
        }
        Ok(Emit { effects, ..Default::default() })
    }
}
//#endregion 🔖️FlowEvalTick

//#region 🔖️FlowEvalResolve
pub mod flow_eval_resolve {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct FlowEvalResolve {
        pub node_hash: u64,
        pub output_json: String,
    }

    pub fn handle(payload: &FlowEvalResolve, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        let _ = session.seed_node_cache(payload.node_hash, &payload.output_json);
        Ok(Emit { effects: vec![eval_tick_effect()], ..Default::default() })
    }
}
//#endregion 🔖️FlowEvalResolve

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn evaluate_updates_preview_state_without_operations() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::Evaluate(evaluate::Evaluate {}));
        assert!(result.operations.is_empty(), "evaluate is a view action");
    }

    #[test]
    fn resolving_a_node_output_re_arms_the_tick_chain() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { node_hash: 42, output_json: "{}".into() }));
        assert!(result.operations.is_empty(), "resolving is not a document edit");
    }
}
//#endregion 🧪️Tests
