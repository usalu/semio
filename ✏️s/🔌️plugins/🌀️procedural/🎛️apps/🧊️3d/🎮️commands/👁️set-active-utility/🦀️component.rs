//! 👁️ 👁️ Procedural3d play app commands command — `set-active-utility`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation, Procedural3dPreviewCamera};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String}

/// 🧰️ Host-owned active-utility switch — clears in-progress hover scratch, never emits document
/// operations.
pub fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }, Procedural3dConfigMutation::SetHover { node_id: None }]))
}
