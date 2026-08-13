//! 🪟️ 🧩️ Flow play app commands command — `move-media-node`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::host_operations;
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::{ FlowEvalSession, Widget};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| {
        host.begin_change();
        host.move_widget(&payload.node_id, payload.x, payload.y).is_ok()
    });
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::amend(operations, format!("move-{}", payload.node_id)))
    }
}
