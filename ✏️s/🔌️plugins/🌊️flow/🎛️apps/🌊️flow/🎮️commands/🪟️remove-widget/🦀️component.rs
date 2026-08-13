//! 🪟️ 🧩️ Flow play app commands command — `remove-widget`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::host_operations;
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::{ FlowEvalSession, Widget};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct RemoveWidget {
    pub widget_id: String,
}

pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let target_id = &payload.widget_id;
    let operations = host_operations(doc.snapshot, config, session, |host| host.remove_widget(target_id).is_ok());
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    let remaining: Vec<String> = config.selected_node_ids.iter().filter(|id| *id != target_id).cloned().collect();
    Ok(Emit {
        artifact_mutations: operations,
        config_mutations: vec![FlowConfigMutation::SetSelection { node_ids: remaining, edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }],
        ..Default::default()
    })
}
