//! 🛍️ 🛍️ Flow play app commands command — `set-catalogue-sections`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
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
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn setting_catalogue_sections_emits_no_artifact_mutations() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::SetCatalogueSections(SetCatalogueSections { sections_json: "[]".into() }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
