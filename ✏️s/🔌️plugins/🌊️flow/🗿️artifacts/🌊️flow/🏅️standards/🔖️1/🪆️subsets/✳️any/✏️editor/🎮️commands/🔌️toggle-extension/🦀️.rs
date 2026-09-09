//! 🧩️ 🧩️ Flow play app commands command — `toggle-extension`.

use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
