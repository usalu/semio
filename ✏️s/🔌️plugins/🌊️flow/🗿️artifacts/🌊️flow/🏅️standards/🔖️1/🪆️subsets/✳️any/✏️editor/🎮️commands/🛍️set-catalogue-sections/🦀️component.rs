//! 🛍️ 🛍️ Flow play app commands command — `set-catalogue-sections`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SetCatalogueSections {
    pub sections_json: String,
}

pub fn handle(payload: &SetCatalogueSections, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::config(vec![FlowConfigMutation::SetCatalogueSections { sections_json: payload.sections_json.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app};
    use crate::editor::flow::FlowCommand;

    #[semio_framework_async_macros::async_test]
    async fn setting_catalogue_sections_emits_no_artifact_mutations() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::SetCatalogueSections(SetCatalogueSections { sections_json: "[]".into() }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
