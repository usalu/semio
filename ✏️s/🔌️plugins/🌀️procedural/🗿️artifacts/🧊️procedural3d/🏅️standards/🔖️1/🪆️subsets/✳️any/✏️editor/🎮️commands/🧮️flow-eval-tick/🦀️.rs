//! 🧮️ 🧮️ Procedural3d play app commands command — `flow-eval-tick`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::{flow_host_with_session, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {}

pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = flow_host_with_session(fixture, session);
    let more = session.tick(&mut host);
    let mut effects = if more { vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(103), action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
    let eval_json = session.eval_json().to_string();
    if let Some(pending) = host.take_pending_extension_eval() {
        let request_json = serde_json::json!({
            "operatorId": pending.operator_id,
            "inputJson": pending.input_json,
            "nodeHash": pending.node_hash})
        .to_string();
        effects.push(Effect::InvokeExtension { req: semio_framework_plugin::RequestId(104), extension_id: pending.extension_id, capability: "evaluate".into(), request_json });
    } else if !more {
        effects.extend(crate::editor::procedural3d::preview_tessellate_effects(session, &eval_json, fixture, cfg.snapshot));
    }
    let config_mutations = vec![Procedural3dConfigMutation::SetPreviewEval { eval_text: (!eval_json.is_empty()).then_some(eval_json) }];
    Ok(Emit { effects, config_mutations, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, dispatch};
    use crate::editor::procedural3d::Procedural3dCommand;

    #[test]
    fn flow_eval_tick_does_not_panic_with_nothing_pending() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Procedural3dCommand::FlowEvalTick(FlowEvalTick {}));
    }
}
//#endregion 🧪️Tests
