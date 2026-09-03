//! 🧮️ 🧮️ Generation3d play app commands command — `flow-eval-tick`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::{flow_host_with_session, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {}

pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = flow_host_with_session(fixture, session);
    let more = session.tick(&mut host);
    let mut effects = if more { vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(103), action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
    let eval_json = session.eval_json().to_string();
    if let Some(pending) = host.take_pending_extension_eval() {
        let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
            ("operatorId".to_string(), dsl::DslValue::String(pending.operator_id.clone())),
            ("inputJson".to_string(), dsl::DslValue::String(pending.input_json.clone())),
            ("nodeHash".to_string(), dsl::DslValue::uint(pending.node_hash)),
        ]));
        effects.push(Effect::InvokeExtension { req: semio_framework_plugin::RequestId(104), extension_id: pending.extension_id, capability: "evaluate".into(), request_json });
    } else if !more {
        effects.extend(crate::editor::generation3d::preview_tessellate_effects(session, &eval_json, fixture, cfg.snapshot));
    }
    let config_mutations = vec![Generation3dConfigMutation::SetPreviewEval { eval_text: (!eval_json.is_empty()).then_some(eval_json) }];
    Ok(Emit { effects, config_mutations, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation3d::testkit::{app, dispatch};
    use crate::editor::generation3d::Generation3dCommand;

    #[test]
    fn flow_eval_tick_does_not_panic_with_nothing_pending() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Generation3dCommand::FlowEvalTick(FlowEvalTick {}));
    }
}
//#endregion 🧪️Tests
