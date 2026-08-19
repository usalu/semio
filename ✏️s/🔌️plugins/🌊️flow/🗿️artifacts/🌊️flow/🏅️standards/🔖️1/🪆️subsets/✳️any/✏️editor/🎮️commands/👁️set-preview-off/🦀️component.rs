//! 👁️ 👁️ Flow play app commands command — `set-preview-off`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SetPreviewOff {
    pub ids: Vec<String>,
    pub value: bool,
}

pub async fn handle(payload: &SetPreviewOff, _doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let mut next = cfg.snapshot.preview_off_node_ids.clone();
    if payload.value {
        for id in &payload.ids {
            if !next.contains(id) {
                next.push(id.clone());
            }
        }
    } else {
        next.retain(|id| !payload.ids.contains(id));
    }
    Ok(Emit::config(vec![FlowConfigMutation::SetPreviewOff { node_ids: next }]))
}
