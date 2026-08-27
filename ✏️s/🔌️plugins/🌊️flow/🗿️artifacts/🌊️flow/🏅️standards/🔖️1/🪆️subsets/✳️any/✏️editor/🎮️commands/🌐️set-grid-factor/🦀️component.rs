//! 🌐️ 🔳️ Flow play app commands command — `set-grid-factor`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SetGridFactor {
    pub value: f64,
}

/// 🔳️ Clamped to the slider's own `0.5..=50.0` range so a scripted dispatch can't desynchronize the
/// control from the config.
pub fn handle(payload: &SetGridFactor, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::config(vec![FlowConfigMutation::SetGridFactor { value: payload.value.clamp(0.5, 50.0) }]))
}
