//! 🕸️ 🕸️ Procedural2d play app commands command — `node-graph-edit`.

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_operations;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::{CameraJson, FlowEvalSession, FlowFixture};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String}

pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
    let selected = cfg.snapshot.selected_ids.clone();
    let mut cleared = false;
    let operations = host_operations(fixture, |host| {
        for operation in &sub_operations {
            match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                "setFixture" => {
                    if let Some(fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<FlowFixture>(json).ok()) {
                        host.replace_fixture(fixture);
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
    });
    let config_mutations = if cleared { vec![Procedural2dConfigMutation::SetSelection { ids: Vec::new() }] } else { Vec::new() };
    Ok(Emit { artifact_mutations: operations, config_mutations, ..Default::default() })
}
