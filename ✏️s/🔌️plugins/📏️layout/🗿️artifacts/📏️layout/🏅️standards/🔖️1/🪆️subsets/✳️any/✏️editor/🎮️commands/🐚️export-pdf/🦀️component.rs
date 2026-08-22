//! 🐚️ 🐚️ Layout play app commands command — `export-pdf`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-pdf")]
pub struct ExportPdf {
    pub page_id: Option<String>,
}

pub async fn handle(_payload: &ExportPdf, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Err(Fault::from("layout-export-job-only"))
}
