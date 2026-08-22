//! 🐚️ 🐚️ Layout play app commands command — `export-png`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-png")]
pub struct ExportPng {
    pub page_id: Option<String>,
}

pub async fn handle(_payload: &ExportPng, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Err(Fault::from("layout-export-job-only"))
}
