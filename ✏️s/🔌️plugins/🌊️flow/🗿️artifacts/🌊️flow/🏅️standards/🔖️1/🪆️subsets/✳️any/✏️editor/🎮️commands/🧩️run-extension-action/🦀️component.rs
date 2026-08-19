//! 🧩️ 🧩️ Flow play app commands command — `run-extension-action`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::commands::evaluate::evaluate_result;
use crate::editor::flow::commands::reorganize::reorganize_operations;
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Registry
/// 🧩️ Built-in flow automations: (id, name, actionId, actionTitle, effect).
pub const FLOW_AUTOMATIONS: &[(&str, &str, &str, &str, &str)] =
    &[("auto-layout", "Auto Layout", "flow.extension.reorganize", "Reorganize Canvas", "reorganize"), ("auto-evaluate", "Auto Evaluate", "flow.extension.evaluate", "Evaluate Fixture", "evaluate")];
//#endregion 🔖️Registry

//#region 🔖️ToggleExtension
//#endregion 🔖️ToggleExtension

//#region 🔖️RunExtensionAction
//#endregion 🔖️RunExtensionAction

//#region 🔖️SetContributions
//#endregion 🔖️SetContributions

/// 🧩️ Dynamic extension-provided action — `action_id` is resolved at runtime against
/// [`super::FLOW_AUTOMATIONS`]; declared `in_palette: false` in the manifest.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct RunExtensionAction {
    pub action_id: String,
}

pub async fn handle(payload: &RunExtensionAction, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
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
