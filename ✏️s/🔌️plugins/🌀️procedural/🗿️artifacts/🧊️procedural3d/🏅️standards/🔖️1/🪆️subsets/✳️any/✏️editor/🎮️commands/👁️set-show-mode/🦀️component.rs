//! 👁️ 👁️ Procedural3d play app commands command — `set-show-mode`.

use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation, Procedural3dPreviewCamera};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "show-mode")]
pub struct SetShowMode {
    pub value: String}

pub fn handle(payload: &SetShowMode, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetShowMode { value: payload.value.clone() }]))
}
