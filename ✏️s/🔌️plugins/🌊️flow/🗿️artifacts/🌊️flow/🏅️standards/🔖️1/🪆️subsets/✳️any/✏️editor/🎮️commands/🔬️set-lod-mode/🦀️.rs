//! 🔭️ 🔭️ Flow play app commands command — `set-lod-mode`.

use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::{op::FlowMutation, FlowSnapshot};
use flow::{FlowEvalSession, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_artifact_infinite_dag::DagDrawLod;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct SetLodMode {
    pub value: String,
}

/// 🎚️ Unknown lod ids are rejected outright (rather than clamped) — the select control only ever
/// offers `FLOW_LOD_MODE_AUTOMATIC` plus the real `DagDrawLod` ids.
pub fn handle(payload: &SetLodMode, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    if payload.value == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(&payload.value).is_some() {
        Ok(Emit::config(vec![FlowConfigMutation::SetLodMode { value: payload.value.clone() }]))
    } else {
        Ok(Emit::default())
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
