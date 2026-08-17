//! 🧮️ 🧮️ Procedural2d play app commands command — `flow-eval-tick`.

use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_from_fixture_with_session;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-tick")]
pub struct FlowEvalTick {}

pub fn handle(_payload: &FlowEvalTick, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = host_from_fixture_with_session(fixture, session);
    let more = session.tick(&mut host);
    let mut effects = if more { vec![semio_framework::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() };
    if let Some(pending) = host.take_pending_extension_eval() {
        let request_json = serde_json::json!({
            "operatorId": pending.operator_id,
            "inputJson": pending.input_json,
            "nodeHash": pending.node_hash})
        .to_string();
        effects.push(semio_framework::kernel::HostEffect::InvokeExtension {
            extension_id: pending.extension_id,
            capability: "evaluate".into(),
            request_json,
            response_action: "flowEvalResolve".into()});
    }
    Ok(Emit { effects, ..Default::default() })
}
