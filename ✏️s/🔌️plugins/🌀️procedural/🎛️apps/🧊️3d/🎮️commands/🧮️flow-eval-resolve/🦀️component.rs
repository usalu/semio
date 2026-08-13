//! 🧮️ 🧮️ Procedural3d play app commands command — `flow-eval-resolve`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{flow_host_with_session, FlowEvalSession};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "flow-eval-resolve")]
pub struct FlowEvalResolve {
    pub node_hash: u64,
    pub output_json: String}

pub fn handle(payload: &FlowEvalResolve, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let _ = session.seed_node_cache(payload.node_hash, &payload.output_json);
    Ok(Emit { effects: vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }], ..Default::default() })
}
