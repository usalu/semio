//! 🧮️ 🧮️ Generation2d play app commands command — `flow-eval-tick`.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::host_from_fixture_with_session;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, ExtensionInvocation, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {}

pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = host_from_fixture_with_session(fixture, session);
    let more = session.tick(&mut host);
    let effects = if more { vec![semio_framework::kernel::Effect::DispatchAction { req: semio_framework_plugin::RequestId(100), action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
    let mut extension_invocations = Vec::new();
    if let Some(pending) = host.take_pending_extension_eval() {
        let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
            ("operatorId".to_string(), dsl::DslValue::String(pending.operator_id.clone())),
            ("inputJson".to_string(), dsl::DslValue::String(pending.input_json.clone())),
            ("nodeHash".to_string(), dsl::DslValue::uint(pending.node_hash)),
        ]));
        extension_invocations.push(ExtensionInvocation::new(pending.extension_id, "evaluate", request_json, "flowEvalResolve"));
    }
    Ok(Emit { effects, extension_invocations, ..Default::default() })
}
