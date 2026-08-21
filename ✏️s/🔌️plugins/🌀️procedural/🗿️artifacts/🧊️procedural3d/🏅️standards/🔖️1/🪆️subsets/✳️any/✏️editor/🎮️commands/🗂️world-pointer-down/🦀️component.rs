//! 🗂️ 🗂️ Procedural3d play app commands command — `world-pointer-down`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "world-pointer-down")]
pub struct WorldPointerDown {}

pub async fn handle(_payload: &WorldPointerDown, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(Emit::default())
}
