//! 🐚️ 🐚️ Layout play app commands command — `export-svg`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-svg")]
pub struct ExportSvg {
    pub page_id: Option<String>,
}

pub async fn handle(_payload: &ExportSvg, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Err(Fault::from("layout-export-job-only"))
}
