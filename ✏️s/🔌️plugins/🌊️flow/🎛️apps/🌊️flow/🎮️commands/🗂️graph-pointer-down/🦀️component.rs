//! 🗂️ 🗂️ Flow play app commands command — `graph-pointer-down`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::{focus_selection_camera, host_operations, sync_host_selection_domains};
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🖱️ A bare pointer-down on empty canvas — drops the NODE selection only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct GraphPointerDown {}

pub fn handle(_payload: &GraphPointerDown, _doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let config = cfg.snapshot;
    Ok(Emit::config(vec![FlowConfigMutation::SetSelection { node_ids: Vec::new(), edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }]))
}
