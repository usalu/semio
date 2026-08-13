//! 🌐️ 🔳️ Flow play app commands command — `set-grid-snap-enabled`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SetGridSnapEnabled {
    pub pressed: Option<bool>,
}

pub fn handle(payload: &SetGridSnapEnabled, _doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::config(vec![FlowConfigMutation::SetGridSnapEnabled { value: payload.pressed.unwrap_or(!cfg.snapshot.grid_snap_enabled) }]))
}
