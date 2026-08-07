//! 🛍️ Flow play app commands — host-supplied extra catalogue sections.
//! The sections themselves are rendered by `📌️panels/🛍️catalogue`.

use crate::apps::flow::config::{FlowConfig, FlowConfigOperation};
use crate::artifacts::flow::{op::FlowOperation, FlowFixture};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetCatalogueSections
pub mod set_catalogue_sections {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-catalogue-sections")]
    pub struct SetCatalogueSections {
        pub sections_json: String,
    }

    pub fn handle(payload: &SetCatalogueSections, _doc: &DocumentView<'_, FlowFixture>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        Ok(Emit::config(vec![FlowConfigOperation::SetCatalogueSections { sections_json: payload.sections_json.clone() }]))
    }
}
//#endregion 🔖️SetCatalogueSections

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn setting_catalogue_sections_emits_no_document_operations() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::SetCatalogueSections(set_catalogue_sections::SetCatalogueSections { sections_json: "[]".into() }));
        assert!(result.operations.is_empty());
    }
}
//#endregion 🧪️Tests
