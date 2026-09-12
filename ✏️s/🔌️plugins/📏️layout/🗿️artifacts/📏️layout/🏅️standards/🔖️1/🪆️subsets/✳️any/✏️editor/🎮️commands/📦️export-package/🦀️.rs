//! 🐚️ 🐚️ Layout play app commands command — `export-package`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-package")]
pub struct ExportPackage {}

pub fn handle(_payload: &ExportPackage, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("layout-export-job-only"))
}
