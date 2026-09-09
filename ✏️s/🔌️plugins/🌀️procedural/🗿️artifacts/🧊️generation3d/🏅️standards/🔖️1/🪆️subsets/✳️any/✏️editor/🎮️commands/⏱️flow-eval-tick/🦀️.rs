//! 🧮️ 🧮️ Generation3d play app commands command — `flow-eval-tick`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{flow_host_with_session, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, ExtensionInvocation, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {}

pub fn evaluate(doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<(Emit<Generation3dMutation, Generation3dConfigMutation>, Option<String>), Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = flow_host_with_session(fixture, session);
    let more = session.tick(&mut host);
    let effects = if more { vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(103), action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
    let eval_json = session.eval_json().to_string();
    let pending_extension_eval = host.take_pending_extension_eval();
    // 🧹️ The host's cloned fixture owns retirement-tracked ordered maps and is dead from here on —
    // close it before the invocation build, never leave it to drop glue.
    host.retire_cold();
    let mut extension_invocations = Vec::new();
    if let Some(pending) = pending_extension_eval {
        let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
            ("operatorId".to_string(), dsl::DslValue::String(pending.operator_id.clone())),
            ("inputJson".to_string(), dsl::DslValue::String(pending.input_json.clone())),
            ("nodeHash".to_string(), dsl::DslValue::uint(pending.node_hash)),
        ]));
        extension_invocations.push(ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve"));
    } else if !more {
        extension_invocations.extend(crate::editor::generation3d::preview_tessellate_invocations(session, &eval_json, fixture, cfg.snapshot));
    }
    let eval_text = (!eval_json.is_empty()).then_some(eval_json);
    Ok((Emit { effects, extension_invocations, ..Default::default() }, eval_text))
}

pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    evaluate(doc, cfg, session).map(|(emit, _)| emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
