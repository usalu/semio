//! 🐚️ 🐚️ Layout play app commands command — `export-png`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-png")]
pub struct ExportPng {
    pub page_id: Option<String>,
}

pub fn handle(_payload: &ExportPng, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("layout-export-job-only"))
}
