//! 📤️ 📤️ Forms play app commands command — `export-fixture`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::artifacts::forms::dsl as forms_dsl;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, Effect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "export-fixture")]
pub struct ExportFixture {}

pub async fn handle(_payload: &ExportFixture, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let data = forms_dsl::print_dsl(spec);
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: format!("{}.forms.dsl", spec.id), mime_type: "text/plain".into(), data, encoding: None }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{dispatch, forms_app};
    use crate::editor::forms::FormsCommand;
    use ExportFixture;

    #[test]
    async fn export_fixture_downloads_the_forms_dsl_text() {
        let mut app = forms_app();
        let result = dispatch(&mut app, FormsCommand::ExportFixture(ExportFixture {}));
        assert!(!result.requested_effects.is_empty(), "exportFixture must emit a host effect");
    }
}
//#endregion 🧪️Tests
