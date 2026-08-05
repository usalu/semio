//! 🧮️ Procedural2d play app commands — off-main-thread flow evaluation driver.

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigOperation};
use crate::artifacts::procedural2d::engine::host_from_fixture_with_session;
use crate::artifacts::procedural2d::op::Procedural2dOperation;
use crate::artifacts::procedural2d::Procedural2dDocument;
use flow_core::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetEvalOutputs
pub mod set_eval_outputs {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-eval-outputs")]
    pub struct SetEvalOutputs {
        pub outputs_json: String,
    }

    pub fn handle(payload: &SetEvalOutputs, _doc: &DocumentView<'_, Procedural2dDocument>, _cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dOperation, Procedural2dConfigOperation>, Fault> {
        session.set_eval_json(payload.outputs_json.clone());
        Ok(Emit::default())
    }
}
//#endregion 🔖️SetEvalOutputs

//#region 🔖️FlowEvalTick
pub mod flow_eval_tick {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "flow-eval-tick")]
    pub struct FlowEvalTick {}

    pub fn handle(_payload: &FlowEvalTick, doc: &DocumentView<'_, Procedural2dDocument>, _cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dOperation, Procedural2dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let mut host = host_from_fixture_with_session(fixture, session);
        let more = session.tick(&mut host);
        let mut effects = if more { vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
        if let Some(pending) = host.take_pending_extension_eval() {
            if let Some(plugin_id) = flow_core::flow_extension_plugin_id(&pending.extension_id) {
                let request_json = serde_json::json!({
                    "operatorId": pending.operator_id,
                    "inputJson": pending.input_json,
                    "nodeHash": pending.node_hash,
                })
                .to_string();
                effects.push(semio_framework_core::kernel::HostEffect::RequestPluginExchange {
                    plugin_id,
                    app_id: "flow-extension-eval".into(),
                    request_json,
                    response_action: "flowEvalResolve".into(),
                });
            }
        }
        Ok(Emit { effects, ..Default::default() })
    }
}
//#endregion 🔖️FlowEvalTick

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, dispatch};
    use crate::apps::procedural2d::Procedural2dCommand;

    #[test]
    fn set_eval_outputs_does_not_mutate_the_document() {
        let mut app = app();
        let before = app.projection().expect("projection").clone();
        dispatch(&mut app, Procedural2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs { outputs_json: "{}".into() }));
        assert_eq!(app.projection().expect("projection"), &before);
    }
}
//#endregion 🧪️Tests
