//! 🗂️ 🗂️ Flow play app commands command — `select-all`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::{focus_selection_camera, host_operations, sync_host_selection_domains};
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SelectAll {}

pub fn handle(_payload: &SelectAll, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let ids: Vec<String> = doc.snapshot.to_fixture().widgets.iter().map(widget_id).map(str::to_string).collect();
    Ok(Emit::config(vec![FlowConfigMutation::SetSelection { node_ids: ids, edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }]))
}
