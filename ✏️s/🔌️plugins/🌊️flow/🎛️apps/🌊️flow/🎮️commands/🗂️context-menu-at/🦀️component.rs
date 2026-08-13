//! 🗂️ 🗂️ Flow play app commands command — `context-menu-at`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::{focus_selection_camera, host_operations, sync_host_selection_domains};
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ContextMenuAt {
    pub id: String,
}

pub fn handle(payload: &ContextMenuAt, _doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    if payload.id.is_empty() {
        return Ok(Emit::default());
    }
    let config = cfg.snapshot;
    Ok(Emit::config(vec![FlowConfigMutation::SetSelection { node_ids: vec![payload.id.clone()], edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }]))
}
