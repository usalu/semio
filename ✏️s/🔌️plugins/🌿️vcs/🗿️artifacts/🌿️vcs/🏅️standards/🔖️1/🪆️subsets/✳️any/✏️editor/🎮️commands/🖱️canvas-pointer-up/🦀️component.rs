//! 🖱️ 🖱️ VCS play app commands command — `canvas-pointer-up`.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {}

pub async fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(Emit::default())
}
