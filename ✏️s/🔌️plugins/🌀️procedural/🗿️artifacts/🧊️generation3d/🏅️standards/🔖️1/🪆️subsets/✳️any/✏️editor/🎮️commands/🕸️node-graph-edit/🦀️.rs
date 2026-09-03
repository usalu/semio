//! 🕸️ 🕸️ Generation3d play app commands command — `node-graph-edit`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::{FlowEvalSession, FlowFixture};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

/// 🌉️ `operations_json` is a locally-defined array of sub-operation descriptors (not a framework
/// boundary type) — parsed generically via `pack::json`'s raw tree, not `serde_json`.
fn parse_sub_operations(text: &str) -> Vec<dsl::json::Value> {
    dsl::json::parse(text).ok().and_then(|value| value.as_array().cloned()).unwrap_or_default()
}

fn apply_operations(fixture: &FlowFixture, sub_operations: &[dsl::json::Value], selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let mut host = host_from_fixture(fixture);
    for operation in sub_operations {
        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
            "setFixture" => {
                if let Some(new_fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| flow::os_pack::json::from_json_str::<FlowFixture>(json).ok()) {
                    host.replace_fixture(new_fixture);
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
    let operations = commit_fixture(fixture, &host.fixture);
    Emit { artifact_mutations: operations, ..Default::default() }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, ctx)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// reachable only through that macro-generated path (`Generation3dPlayApp::handle` always routes this
/// command through `apply` below instead), so `"deleteSelection"` sub-operations degrade to treating
/// the selection as empty.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let sub_operations = parse_sub_operations(&payload.operations_json);
    Ok(apply_operations(&doc.snapshot.fixture, &sub_operations, &[]))
}

/// 🕹️ `"deleteSelection"` reads the `graph` domain's current selection instead of a deleted config
/// field — no config mutation needed afterwards, the framework auto-prunes the deleted ids out of
/// `graph`'s selection.
pub fn apply(
    payload: &NodeGraphEdit,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    interaction: &InteractionView<'_>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let sub_operations = parse_sub_operations(&payload.operations_json);
    Ok(apply_operations(&doc.snapshot.fixture, &sub_operations, &interaction.selection("graph").ids))
}
