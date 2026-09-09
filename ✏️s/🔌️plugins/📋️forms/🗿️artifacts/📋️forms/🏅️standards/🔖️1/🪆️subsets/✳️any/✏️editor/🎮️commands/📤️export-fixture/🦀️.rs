//! 📤️ 📤️ Forms play app commands command — `export-fixture`.

use crate::document_dsl as forms_dsl;
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-fixture")]
pub struct ExportFixture {}

pub fn handle(_payload: &ExportFixture, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let data = forms_dsl::print_dsl(spec);
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{}.forms.dsl", spec.id), mime_type: "text/plain".into(), data, encoding: None }))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
