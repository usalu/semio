//! 🖱️ 🖱️ VCS play app commands command — `canvas-wheel`.

use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-wheel")]
pub struct CanvasWheel {}

pub async fn handle(_payload: &CanvasWheel, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(Emit::default())
}
