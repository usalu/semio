//! 🧩️ 🧩️ Flow play app commands command — `run-extension-action`.

use crate::editor::flow::commands::evaluate::evaluate_result;
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

pub fn handle(payload: &RunExtensionAction, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let Some((id, _, _, _, effect)) = FLOW_AUTOMATIONS.iter().find(|(_, _, entry_action_id, ..)| *entry_action_id == payload.action_id) else {
        return Ok(Emit::default());
    };
    if !&crate::editor::flow::modes::edit::windows::main::config::current(cfg).automation_enabled().get(*id).copied().unwrap_or(false) {
        return Ok(Emit::default());
    }
    match *effect {
        "reorganize" => Ok(Emit::mutations(reorganize_operations(doc, cfg, session))),
        "evaluate" => Ok(evaluate_result(doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session)),
        _ => Ok(Emit::default()),
    }
}
