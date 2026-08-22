//! 🧮️ 🧮️ Procedural3d play app commands command — `flow-tessellate-resolve`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "flow-tessellate-resolve")]
pub struct FlowTessellateResolve {
    pub node_hash: u64,
    pub output_json: String,
}

pub fn handle(payload: &FlowTessellateResolve, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let _ = session.resolve_preview_tessellate(payload.node_hash, &payload.output_json);
    Ok(Emit::default())
}
