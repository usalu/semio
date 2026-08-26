//! 🕸️ 🕸️ Procedural2d play app commands command — `node-graph-edit`.

use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::schema::host_operations;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use flow::{FlowEvalSession, FlowFixture};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

fn apply_operations(fixture: &FlowFixture, sub_operations: &[Value], selected: &[String]) -> Emit<Procedural2dMutation, Procedural2dConfigMutation> {
    let operations = host_operations(fixture, |host| {
        for operation in sub_operations {
            match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                "setFixture" => {
                    if let Some(fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<FlowFixture>(json).ok()) {
                        host.replace_fixture(fixture);
                    }
                }
                "deleteSelection" => {
                    for id in selected {
                        let _ = host.remove_widget(id);
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
    Emit { artifact_mutations: operations, ..Default::default() }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Procedural2dPlayApp::handle` always routes this
/// command through `apply` below instead), so `"deleteSelection"` sub-operations degrade to treating
/// the selection as empty.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    apply_selected(payload, doc, &[])
}

pub fn apply_selected(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Procedural2dSnapshot>, selected: &[String]) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
    Ok(apply_operations(&doc.snapshot.fixture, &sub_operations, selected))
}

/// 🕹️ `"deleteSelection"` reads the `graph` domain's current selection instead of a deleted config
/// field — no config mutation needed afterwards, the framework auto-prunes the deleted ids out of
/// `graph`'s selection.
pub fn apply(
    payload: &NodeGraphEdit,
    doc: &ArtifactView<'_, Procedural2dSnapshot>,
    _cfg: &ConfigView<'_, Procedural2dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    apply_selected(payload, doc, &interaction.selection("graph").ids)
}
