//! 🧩️ Flow play app commands — the built-in extension registry and its enable/run verbs.
//!
//! `FLOW_AUTOMATIONS` is the single source of truth for the built-in extension palette: the catalogue
//! panel renders it (`📌️panels/🛍️catalogue`) and `run_extension_action` resolves an incoming action id
//! against it. An extension action only runs while its automation is enabled in the config.

use crate::apps::flow::commands::{eval::evaluate_result, layout::reorganize_operations};
use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Registry
/// 🧩️ Built-in flow automations: (id, name, actionId, actionTitle, effect).
pub const FLOW_AUTOMATIONS: &[(&str, &str, &str, &str, &str)] =
    &[("auto-layout", "Auto Layout", "flow.extension.reorganize", "Reorganize Canvas", "reorganize"), ("auto-evaluate", "Auto Evaluate", "flow.extension.evaluate", "Evaluate Fixture", "evaluate")];
//#endregion 🔖️Registry

//#region 🔖️ToggleExtension
pub mod toggle_extension {
    use super::*;

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
}
//#endregion 🔖️ToggleExtension

//#region 🔖️RunExtensionAction
pub mod run_extension_action {
    use super::*;

    /// 🧩️ Dynamic extension-provided action — `action_id` is resolved at runtime against
    /// [`super::FLOW_AUTOMATIONS`]; declared `in_palette: false` in the manifest.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct RunExtensionAction {
        pub action_id: String,
    }

    pub fn handle(payload: &RunExtensionAction, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        let Some((id, _, _, _, effect)) = FLOW_AUTOMATIONS.iter().find(|(_, _, entry_action_id, ..)| *entry_action_id == payload.action_id) else {
            return Ok(Emit::default());
        };
        if !cfg.snapshot.automation_enabled().get(*id).copied().unwrap_or(false) {
            return Ok(Emit::default());
        }
        match *effect {
            "reorganize" => Ok(Emit::mutations(reorganize_operations(doc, cfg, session))),
            "evaluate" => Ok(evaluate_result(doc.snapshot, cfg.snapshot, session)),
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RunExtensionAction


//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    /// 🧩️ Host-pushed contribution catalogue JSON.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct SetContributions {
        pub json: String,
    }

    pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(Emit::config(vec![FlowConfigMutation::SetContributions { json: payload.json.clone() }]))
    }
}
//#endregion 🔖️SetContributions

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn toggle_extension_and_run_action_reorganizes_fixture() {
        let mut app = flow_app();
        let before = app.snapshot().expect("snapshot").widgets.len();
        let ignored = dispatch(&mut app, FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }));
        assert!(ignored.mutations.is_empty(), "disabled automation action must be a no-operation");
        dispatch(&mut app, FlowCommand::ToggleExtension(toggle_extension::ToggleExtension { id: "auto-layout".into(), enabled: true }));
        dispatch(&mut app, FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }));
        assert_eq!(app.snapshot().expect("snapshot").widgets.len(), before, "reorganize keeps every widget");
    }

    #[test]
    fn an_unknown_extension_action_id_is_a_no_operation() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "third.party.nope".into() }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
