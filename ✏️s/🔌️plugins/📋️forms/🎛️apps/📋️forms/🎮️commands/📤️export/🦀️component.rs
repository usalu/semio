//! 📤️ Forms play app commands — the `.forms` DSL fixture export shell effect (host round-trip, no
//! document operations either way).

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
// 🧷️ Aliased: `ExportFixture` below derives the EXTERN `dsl` crate's `dsl::DslRecord` — importing the
// artifact's own `dsl` submodule under the bare name would shadow it.
use crate::artifacts::forms::dsl as forms_dsl;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

//#region 🔖️ExportFixture
pub mod export_fixture {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-fixture")]
    pub struct ExportFixture {}

    pub fn handle(_payload: &ExportFixture, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let spec = doc.snapshot;
        let data = forms_dsl::print_dsl(spec);
        Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{}.forms.dsl", spec.id), mime_type: "text/plain".into(), data, encoding: None }))
    }
}
//#endregion 🔖️ExportFixture

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app};
    use crate::apps::forms::FormsCommand;
    use export_fixture::ExportFixture;

    #[test]
    fn export_fixture_downloads_the_forms_dsl_text() {
        let mut app = forms_app();
        let result = dispatch(&mut app, FormsCommand::ExportFixture(ExportFixture {}));
        assert!(!result.requested_effects.is_empty(), "exportFixture must emit a host effect");
    }
}
//#endregion 🧪️Tests
