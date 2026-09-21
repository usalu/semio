//! 🕸️ 🎯️ Flow play app commands command — `node-graph-edit`.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::{fallible_host_operations, flow_graph_selection_domains, sync_host_selection, FLOW_GRAPH_OPERATION_RAW_BYTES, FLOW_INTERACTION_GRAPH, FLOW_STORE_MAX_MUTATION_ITEMS};
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️FlowNodeGraphEditOp
/// 🎯️ One batched edit inside a `FlowCommand::NodeGraphEdit`/`SpotlightCommit`, closed over the exact
/// operation rows published by the shared NodeGraph renderer.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum)]
pub enum FlowNodeGraphEditOp {
    #[dsl(key = "set-host-snapshot")]
    SetHostSnapshot { host_snapshot_json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    #[dsl(key = "disconnect")]
    Disconnect { synapse_id: String },
    #[dsl(key = "move")]
    Move { node_id: String, x: f64, y: f64 },
}

fn action_row_fields<'a>(row: &'a dsl::DslValue, operation: &str, expected: &[&str]) -> Result<&'a [(String, dsl::DslValue)], Fault> {
    let fields = row.as_object().ok_or_else(|| Fault::from(format!("nodeGraphEdit {operation} row must be an object")))?;
    if fields.len() != expected.len() || fields.iter().any(|(field, _)| !expected.contains(&field.as_str())) || expected.iter().any(|expected| fields.iter().filter(|(field, _)| field.as_str() == *expected).count() != 1) {
        return Err(Fault::from(format!("nodeGraphEdit {operation} row has fields outside its closed schema")));
    }
    Ok(fields)
}

fn action_string(row: &dsl::DslValue, operation: &str, field: &str) -> Result<String, Fault> {
    row.get(field).and_then(dsl::DslValue::as_str).map(str::to_string).ok_or_else(|| Fault::from(format!("nodeGraphEdit {operation}.{field} must be a string")))
}

fn action_id(row: &dsl::DslValue, operation: &str, field: &str) -> Result<String, Fault> {
    action_string(row, operation, field).and_then(|value| (!value.is_empty()).then_some(value).ok_or_else(|| Fault::from(format!("nodeGraphEdit {operation}.{field} must not be empty"))))
}

fn action_number(row: &dsl::DslValue, operation: &str, field: &str) -> Result<f64, Fault> {
    row.get(field).and_then(dsl::DslValue::as_f64).filter(|value| value.is_finite()).ok_or_else(|| Fault::from(format!("nodeGraphEdit {operation}.{field} must be a finite number")))
}

fn action_host_snapshot_json(row: &dsl::DslValue, operation: &str) -> Result<String, Fault> {
    let encoded = action_string(row, operation, "hostSnapshotJson")?;
    let json: serde_json::Value = serde_json::from_str(&encoded).map_err(|_| Fault::from("nodeGraphEdit setHostSnapshot.hostSnapshotJson must be valid JSON"))?;
    let snapshot: semio_framework_artifact_flow_flow::FlowHostSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(json)).map_err(|_| Fault::from("nodeGraphEdit setHostSnapshot.hostSnapshotJson must encode a Flow host snapshot"))?;
    snapshot.retire_cold();
    Ok(encoded)
}

/// 🧾 Decodes the renderer's current externally-tagged row schema. Every row must decode or the
/// entire batch is refused before a retained operation is admitted.
pub fn operations_from_action(args: &dsl::DslValue) -> Result<Vec<FlowNodeGraphEditOp>, Fault> {
    let root = args.as_object().ok_or_else(|| Fault::from("nodeGraphEdit arguments must be an object"))?;
    if root.len() != 1 || root[0].0 != "operations" {
        return Err(Fault::from("nodeGraphEdit arguments have fields outside their closed schema"));
    }
    let rows = root[0].1.as_array().ok_or_else(|| Fault::from("nodeGraphEdit operations must be an array"))?;
    if rows.len() > FLOW_STORE_MAX_MUTATION_ITEMS {
        return Err(Fault::from(format!("nodeGraphEdit operations exceed the {FLOW_STORE_MAX_MUTATION_ITEMS}-row authority")));
    }
    let encoded_bytes = dsl::json::to_json_string(args).len();
    if encoded_bytes > FLOW_GRAPH_OPERATION_RAW_BYTES {
        return Err(Fault::from(format!("nodeGraphEdit arguments exceed the {FLOW_GRAPH_OPERATION_RAW_BYTES}-byte wire authority")));
    }
    rows.iter()
        .map(|row| {
            let operation = row.get("operation").and_then(dsl::DslValue::as_str).ok_or_else(|| Fault::from("nodeGraphEdit row.operation must be a string"))?;
            match operation {
                "setHostSnapshot" => {
                    action_row_fields(row, operation, &["operation", "hostSnapshotJson"])?;
                    Ok(FlowNodeGraphEditOp::SetHostSnapshot { host_snapshot_json: action_host_snapshot_json(row, operation)? })
                }
                "deleteSelection" => {
                    action_row_fields(row, operation, &["operation"])?;
                    Ok(FlowNodeGraphEditOp::DeleteSelection)
                }
                "connect" => {
                    action_row_fields(row, operation, &["operation", "sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId"])?;
                    Ok(FlowNodeGraphEditOp::Connect {
                        source_node_id: action_id(row, operation, "sourceNodeId")?,
                        source_port_id: action_string(row, operation, "sourcePortId")?,
                        target_node_id: action_id(row, operation, "targetNodeId")?,
                        target_port_id: action_string(row, operation, "targetPortId")?,
                    })
                }
                "disconnect" => {
                    action_row_fields(row, operation, &["operation", "synapseId"])?;
                    Ok(FlowNodeGraphEditOp::Disconnect { synapse_id: action_id(row, operation, "synapseId")? })
                }
                "move" => {
                    action_row_fields(row, operation, &["operation", "nodeId", "x", "y"])?;
                    Ok(FlowNodeGraphEditOp::Move { node_id: action_id(row, operation, "nodeId")?, x: action_number(row, operation, "x")?, y: action_number(row, operation, "y")? })
                }
                _ => Err(Fault::from(format!("unknown nodeGraphEdit operation `{operation}`"))),
            }
        })
        .collect()
}
//#endregion 🔖️FlowNodeGraphEditOp

//#region 🔖️SharedDispatch
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_nodes` is the "graph"
/// domain's live node selection (read by the caller via `InteractionView`) — no `SetSelection` config
/// mutation afterwards, the framework auto-prunes deleted ids out of `graph`'s selection via
/// `interaction_topology`.
pub fn node_graph_edit_result(
    snapshot: &FlowSnapshot,
    config: &FlowMainWindowConfig,
    session: &FlowEvalSession,
    operations: &[FlowNodeGraphEditOp],
    selected_nodes: &[String],
) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let artifact_mutations = fallible_host_operations(snapshot, config, session, |host| {
        for sub_operation in operations {
            match sub_operation {
                FlowNodeGraphEditOp::SetHostSnapshot { host_snapshot_json } => {
                    let json: serde_json::Value = serde_json::from_str(host_snapshot_json).map_err(|error| Fault::from(format!("nodeGraphEdit setHostSnapshot JSON refusal: {error}")))?;
                    let parsed: semio_framework_artifact_flow_flow::FlowHostSnapshot = dsl::FromValue::from_value(dsl::DslValue::from(json))
                        .map_err(|error| Fault::from(format!("nodeGraphEdit setHostSnapshot schema refusal: {error}")))?;
                    host.begin_change();
                    host.set_host_snapshot_preserving_history(parsed);
                }
                FlowNodeGraphEditOp::DeleteSelection => {
                    sync_host_selection(host, selected_nodes);
                    host.delete_selection().map_err(|error| Fault::from(format!("nodeGraphEdit deleteSelection refusal: {error}")))?;
                }
                FlowNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                    host.connect_ports(source_node_id, source_port_id, target_node_id, target_port_id).map_err(|error| Fault::from(format!("nodeGraphEdit connect refusal: {error}")))?;
                }
                FlowNodeGraphEditOp::Disconnect { synapse_id } => {
                    host.disconnect(synapse_id).map_err(|error| Fault::from(format!("nodeGraphEdit disconnect refusal: {error}")))?;
                }
                FlowNodeGraphEditOp::Move { node_id, x, y } => {
                    host.begin_change();
                    host.move_widget(node_id, *x, *y).map_err(|error| Fault::from(format!("nodeGraphEdit move refusal: {error}")))?;
                }
            }
        }
        Ok(!operations.is_empty())
    })?;
    Ok(Emit::mutations(artifact_mutations))
}
//#endregion 🔖️SharedDispatch

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
pub struct NodeGraphEdit {
    #[dsl(statements)]
    pub operations: Vec<FlowNodeGraphEditOp>,
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot), so it still requires a `handle` of this signature to exist even though
/// it is reachable only through that macro-generated path (`FlowPlayApp::handle` always routes this
/// command through `apply` below instead) — degrades to treating the selection as empty.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    node_graph_edit_result(doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, &payload.operations, &[])
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` has no `interaction` slot (see
/// `delete_selection::apply`'s doc comment) — `FlowPlayApp::handle` routes this command through `apply`.
pub fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession, interaction: &InteractionView<'_>) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let (nodes, _edges) = flow_graph_selection_domains(&interaction.selection(FLOW_INTERACTION_GRAPH).ids);
    node_graph_edit_result(doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, &payload.operations, &nodes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
