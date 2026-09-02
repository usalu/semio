//! 🧮️ 🧮️ Procedural3d play app commands command — `flow-eval-resolve`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-resolve")]
pub struct FlowEvalResolve {
    pub node_hash: u64,
    pub output_json: String,
}

pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let _ = session.seed_node_cache(payload.node_hash, &payload.output_json);
    Ok(Emit { effects: vec![Effect::DispatchAction { req: semio_framework_plugin::RequestId(102), action: "flowEvalTick".into(), args: None, delay_ms: 0 }], ..Default::default() })
}
