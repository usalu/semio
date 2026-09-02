//! 🧮️ 🧮️ Procedural2d play app commands command — `flow-eval-tick`.

use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::schema::host_from_fixture_with_session;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {}

pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = host_from_fixture_with_session(fixture, session);
    let more = session.tick(&mut host);
    let mut effects = if more { vec![semio_framework::kernel::Effect::DispatchAction { req: semio_framework_plugin::RequestId(100), action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
    if let Some(pending) = host.take_pending_extension_eval() {
        let request_json = dsl::json::to_json_string(&dsl::DslValue::object([
            ("operatorId".to_string(), dsl::DslValue::String(pending.operator_id.clone())),
            ("inputJson".to_string(), dsl::DslValue::String(pending.input_json.clone())),
            ("nodeHash".to_string(), dsl::DslValue::uint(pending.node_hash)),
        ]));
        effects.push(semio_framework::kernel::Effect::InvokeExtension { req: semio_framework::kernel::RequestId(101), extension_id: pending.extension_id, capability: "evaluate".into(), request_json });
    }
    Ok(Emit { effects, ..Default::default() })
}
