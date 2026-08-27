//! 🧩️ 🧩️ Flow play app commands command — `toggle-extension`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ToggleExtension {
    pub id: String,
    pub enabled: bool,
}

pub fn handle(payload: &ToggleExtension, _doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let mut map = cfg.snapshot.automation_enabled();
    map.insert(payload.id.clone(), payload.enabled);
    Ok(Emit::config(vec![FlowConfigMutation::SetAutomationEnabled { json: serde_json::to_string(&map).unwrap_or_default() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app};
    use crate::editor::flow::FlowCommand;

    #[semio_framework_async_macros::async_test]
    async fn toggle_extension_and_run_action_reorganizes_fixture() {
        let mut app = flow_app();
        let before = app.snapshot().expect("snapshot").to_fixture().widgets.len();
        let ignored = dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }));
        assert!(ignored.mutations.is_empty(), "disabled automation action must be a no-operation");
        dispatch(&mut app, FlowCommand::ToggleExtension(ToggleExtension { id: "auto-layout".into(), enabled: true }));
        dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }));
        assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before, "reorganize keeps every widget");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_extension_action_id_is_a_no_operation() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "third.party.nope".into() }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
