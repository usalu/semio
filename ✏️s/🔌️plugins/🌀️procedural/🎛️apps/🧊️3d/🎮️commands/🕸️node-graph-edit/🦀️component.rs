//! 🕸️ 🕸️ Procedural3d play app commands command — `node-graph-edit`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{CameraJson, FlowEvalSession, FlowFixture};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String}

pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
    let selected = cfg.snapshot.selected_node_ids.clone();
    let mut host = host_from_fixture(fixture);
    let mut cleared = false;
    for operation in &sub_operations {
        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
            "setFixture" => {
                if let Some(new_fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<FlowFixture>(json).ok()) {
                    host.replace_fixture(new_fixture);
                }
            }
            "deleteSelection" => {
                for id in &selected {
                    if host.remove_widget(id).is_ok() {
                        cleared = true;
                    }
                }
            }
            "connect" => {
                let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                    let _ = host.connect_ports(from, from_port, to, to_port);
                }
            }
            _ => {}
        }
    }
    let operations = commit_fixture(fixture, &host.fixture);
    let config_mutations = if cleared { vec![Procedural3dConfigMutation::SetSelection { node_ids: Vec::new() }] } else { Vec::new() };
    Ok(Emit { artifact_mutations: operations, config_mutations, ..Default::default() })
}
