//! 📤️ Forms play app commands — the `.forms` DSL fixture export shell effect (host round-trip, no
//! document operations either way).

use crate::apps::forms::config::{FormsConfig, FormsConfigOperation};
use crate::artifacts::forms::{dsl, op::FormOperation, FormSpec};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

//#region 🔖️ExportFixture
pub mod export_fixture {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-fixture")]
    pub struct ExportFixture {}

    pub fn handle(_payload: &ExportFixture, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormOperation, FormsConfigOperation>, Fault> {
        let spec = doc.projection;
        let data = dsl::print_dsl(spec);
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
