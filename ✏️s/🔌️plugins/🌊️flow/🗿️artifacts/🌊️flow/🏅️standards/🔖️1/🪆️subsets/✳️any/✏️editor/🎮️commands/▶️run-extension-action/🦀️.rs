//! 🧩️ 🧩️ Flow play app commands command — `run-extension-action`.

use crate::editor::flow::commands::evaluate::evaluate_result;
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use crate::editor::flow::commands::reorganize::reorganize_operations;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Registry
/// 🧩️ Built-in flow automations: (id, name, actionId, actionTitle, effect).
pub const FLOW_AUTOMATIONS: &[(&str, &str, &str, &str, &str)] =
    &[("auto-layout", "Auto Layout", "flow.extension.reorganize", "Reorganize Canvas", "reorganize"), ("auto-evaluate", "Auto Evaluate", "flow.extension.evaluate", "Evaluate Fixture", "evaluate")];
//#endregion 🔖️Registry

//#region 🔖️ToggleExtension
//#endregion 🔖️ToggleExtension

//#region 🔖️RunExtensionAction
//#endregion 🔖️RunExtensionAction


/// 🧩️ Dynamic extension-provided action — `action_id` is resolved at runtime against
/// [`super::FLOW_AUTOMATIONS`]; declared `in_palette: false` in the manifest.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct RunExtensionAction {
    pub action_id: String,
}

/// 🧩️ The extension effect against an already-resolved window config — the one body the batch
/// `handle` below and the retained `FlowGraphOperationWork` route both run, so neither can drift
/// from the other (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn extension_action_result(payload: &RunExtensionAction, snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, NoConfigMutation> {
    let Some((id, _, _, _, effect)) = FLOW_AUTOMATIONS.iter().find(|(_, _, entry_action_id, ..)| *entry_action_id == payload.action_id) else {
        return Emit::default();
    };
    if !config.automation_enabled().get(*id).copied().unwrap_or(false) {
        return Emit::default();
    }
    match *effect {
        "reorganize" => Emit::mutations(reorganize_operations(snapshot, config, session)),
        "evaluate" => evaluate_result(snapshot, config, session),
        _ => Emit::default(),
    }
}

pub fn handle(payload: &RunExtensionAction, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(extension_action_result(payload, doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session))
}
