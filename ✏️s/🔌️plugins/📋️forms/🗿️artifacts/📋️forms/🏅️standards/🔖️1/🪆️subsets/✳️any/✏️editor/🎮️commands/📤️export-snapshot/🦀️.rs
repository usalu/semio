//! 📤️ 📤️ Forms play app commands command — `export-snapshot`.

use crate::standards::v1::subsets::any::io::text::snapshot as forms_dsl;
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-snapshot")]
pub struct ExportSnapshot {}

pub fn handle(_payload: &ExportSnapshot, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let data = forms_dsl::print_dsl(spec);
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{}.forms", spec.id), mime_type: "text/plain;charset=utf-8".into(), data, encoding: None }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
