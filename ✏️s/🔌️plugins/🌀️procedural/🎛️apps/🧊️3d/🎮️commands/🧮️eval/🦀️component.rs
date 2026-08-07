//! 🧮️ Procedural3d play app commands — off-main-thread flow evaluation driver.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation};
use crate::artifacts::procedural3d::op::Procedural3dOperation;
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow::{flow_host_with_session, FlowEvalSession};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

//#region 🔖️FlowEvalTick
pub mod flow_eval_tick {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "flow-eval-tick")]
    pub struct FlowEvalTick {}

    pub fn handle(_payload: &FlowEvalTick, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let mut host = flow_host_with_session(fixture, session);
        let more = session.tick(&mut host);
        let mut effects = if more { vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
        if let Some(pending) = host.take_pending_extension_eval() {
            let request_json = serde_json::json!({
                "operatorId": pending.operator_id,
                "inputJson": pending.input_json,
                "nodeHash": pending.node_hash,
            })
            .to_string();
            effects.push(HostEffect::InvokeExtension {
                extension_id: pending.extension_id,
                capability: "evaluate".into(),
                request_json,
                response_action: "flowEvalResolve".into(),
            });
        } else if !more {
            let eval_json = session.eval_json().to_string();
            effects.extend(crate::artifacts::procedural3d::engine::preview_tessellate_effects(
                session,
                &eval_json,
                fixture,
                cfg.projection,
            ));
        }
        Ok(Emit { effects, ..Default::default() })
    }
}
//#endregion 🔖️FlowEvalTick

//#region 🔖️FlowEvalResolve
pub mod flow_eval_resolve {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "flow-eval-resolve")]
    pub struct FlowEvalResolve {
        pub node_hash: u64,
        pub output_json: String,
    }

    pub fn handle(payload: &FlowEvalResolve, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let _ = session.seed_node_cache(payload.node_hash, &payload.output_json);
        Ok(Emit { effects: vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }], ..Default::default() })
    }
}
//#endregion 🔖️FlowEvalResolve

//#region 🔖️FlowTessellateResolve
pub mod flow_tessellate_resolve {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "flow-tessellate-resolve")]
    pub struct FlowTessellateResolve {
        pub node_hash: u64,
        pub output_json: String,
    }

    pub fn handle(payload: &FlowTessellateResolve, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let _ = session.resolve_preview_tessellate(payload.node_hash, &payload.output_json);
        Ok(Emit::default())
    }
}
//#endregion 🔖️FlowTessellateResolve

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn flow_eval_tick_does_not_panic_with_nothing_pending() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Procedural3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}));
    }
}
//#endregion 🧪️Tests
