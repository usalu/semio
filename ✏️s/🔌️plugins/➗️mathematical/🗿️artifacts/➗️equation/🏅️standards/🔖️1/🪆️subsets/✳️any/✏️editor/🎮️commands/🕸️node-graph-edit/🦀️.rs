//! 🕸️ 🕸️ Equation play app commands command — `node-graph-edit`: the one verb both node-graph hosts dispatch for graph
//! edits (the shared row vocabulary of design §13.3), yielding id-keyed intent leaves (`move-nodes`, `connect-nodes`,
//! `disconnect-nodes`, `delete-node`), never a whole-graph `replace-graph`. Adding a node is the guest's own `addNode` verb.

use crate::editor::equation::{EQUATION_MAX_DELETE_IDS, EQUATION_MAX_TEXT_BYTES};
use crate::op::EquationMutation;
use crate::standards::v1::subsets::graph::schema::mutations::{connect_nodes::ConnectNodes, delete_node::DeleteNode, disconnect_nodes::DisconnectNodes, move_nodes::MoveNodes};
use crate::{EquationEdge, EquationGraph, EquationSnapshot};
use semio_framework_pack_json::Value as JsonValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_tool_machine::{node_drag_emit, NodeGraphEditRow};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// 🪪️ The verb a node-graph tool transaction is scoped by: `<appId>#nodeGraphEdit`.
pub const NODE_GRAPH_EDIT_VERB: &str = "nodeGraphEdit";

/// 🪪️ The editor app every node-graph tool transaction's `tool` is scoped by.
pub const EQUATION_EDITOR_APP_ID: &str = "s.mathematical.equation@1/*#editor";

/// 🎨️ `nodeGraphActions.edit` (`"nodeGraphEdit"`) is the shared renderer-wide action id the generic node-graph canvas
/// dispatches interactive edit gestures under; `operations_json` is the JSON array of its rows.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

/// 🧾️ One decoded `nodeGraphEdit` row the equation graph can carry: the node-graph gesture record `move`, a `connect` of
/// two nodes (an equation edge joins nodes, not ports), a `disconnect` of one edge, and a `delete` of named nodes and edges.
#[derive(Clone)]
pub(crate) enum EquationEditOperation {
    Move { gesture_id: String, node_ids: Vec<String>, dx: f64, dy: f64 },
    Connect { source: String, target: String },
    Disconnect { id: String },
    Delete { node_ids: Vec<String>, edge_ids: Vec<String> },
}

impl EquationEditOperation {
    /// 🚪️ Decodes one row through the ONE shared row decoder of `🛠️tool-machine` within the retained capacities: a malformed
    /// row, an over-capacity id list or text, and the `setSlider`/`insertPort` rows the equation graph has no widget for
    /// are refused by name, so the whole batch is refused before a leaf is yielded.
    pub(crate) fn from_value(value: &JsonValue) -> Result<Self, Fault> {
        let text = |text: &str| if text.len() > EQUATION_MAX_TEXT_BYTES { Err(Fault::from("equation-edit-text-capacity")) } else { Ok(text.to_string()) };
        let ids = |ids: Vec<String>| if ids.len() > EQUATION_MAX_DELETE_IDS || ids.iter().any(|id| id.len() > EQUATION_MAX_TEXT_BYTES) { Err(Fault::from("equation-edit-id-capacity")) } else { Ok(ids) };
        match NodeGraphEditRow::from_row(&semio_framework_pack_json::to_dsl_value(value)).map_err(|reason| Fault::from(format!("equation nodeGraphEdit refusal: {reason}")))? {
            NodeGraphEditRow::Move(record) => Ok(Self::Move { gesture_id: text(&record.gesture_id)?, node_ids: ids(record.node_ids)?, dx: record.dx, dy: record.dy }),
            NodeGraphEditRow::Connect { source_node_id, target_node_id, .. } => Ok(Self::Connect { source: text(&source_node_id)?, target: text(&target_node_id)? }),
            NodeGraphEditRow::Disconnect { synapse_id } => Ok(Self::Disconnect { id: text(&synapse_id)? }),
            NodeGraphEditRow::Delete { node_ids, synapse_ids } => Ok(Self::Delete { node_ids: ids(node_ids)?, edge_ids: ids(synapse_ids)? }),
            NodeGraphEditRow::SetSlider { .. } | NodeGraphEditRow::InsertPort { .. } => Err(Fault::from("equation nodeGraphEdit refusal: the equation graph has no sliders and no variadic ports")),
        }
    }

    /// 📏️ The heap this row retains while a retained job holds it.
    pub(crate) fn retained_bytes(&self) -> usize {
        let ids = |ids: &Vec<String>| ids.capacity() * size_of::<String>() + ids.iter().map(String::capacity).sum::<usize>();
        size_of::<Self>()
            + match self {
                Self::Move { gesture_id, node_ids, .. } => gesture_id.capacity() + ids(node_ids),
                Self::Connect { source, target } => source.capacity() + target.capacity(),
                Self::Disconnect { id } => id.capacity(),
                Self::Delete { node_ids, edge_ids } => ids(node_ids) + ids(edge_ids),
            }
    }
}

/// 🔢️ The first `<prefix><k>` id, counting from `from`, that `taken` does not hold.
fn fresh_id(prefix: char, from: usize, taken: impl Fn(&str) -> bool) -> String {
    (from..).map(|k| format!("{prefix}{k}")).find(|id| !taken(id)).unwrap_or_default()
}

/// 🧮️ The intent leaves ONE row yields against the working `graph`, which it advances by exactly those leaves, plus the
/// press of a node drag:
/// - `move` — the RELATIVE `move-nodes` leaf over the record's nodes the graph holds (none, or a zero offset, is nothing);
/// - `connect` — `connect-nodes` with the first free `e<k>` id, when both endpoints exist and no parallel edge does;
/// - `disconnect` — `disconnect-nodes` of the edge, when the graph holds it;
/// - `delete` — `disconnect-nodes` per named edge and per edge incident to a named node, then `delete-node` per node, so every
///   row is point-invertible (the equation store preparation declares one forward plus one inverse row per item).
pub(crate) fn equation_edit_operation_leaves(graph: &mut EquationGraph, operation: &EquationEditOperation) -> (Vec<EquationMutation>, Option<String>) {
    match operation {
        EquationEditOperation::Move { gesture_id, node_ids, dx, dy } => {
            let ids: Vec<String> = node_ids.iter().filter(|id| graph.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
            if ids.is_empty() || (*dx, *dy) == (0.0, 0.0) {
                return (Vec::new(), None);
            }
            for node in graph.nodes.iter_mut().filter(|node| ids.contains(&node.id)) {
                node.x += dx;
                node.y += dy;
            }
            (vec![EquationMutation::MoveNodes(MoveNodes { ids, dx: *dx, dy: *dy })], Some(gesture_id.clone()))
        }
        EquationEditOperation::Connect { source, target } => {
            let exists = |id: &str| graph.nodes.iter().any(|node| node.id == id);
            if !exists(source) || !exists(target) || graph.edges.iter().any(|edge| &edge.source == source && &edge.target == target) {
                return (Vec::new(), None);
            }
            let id = fresh_id('e', graph.edges.len(), |id| graph.edges.iter().any(|edge| edge.id == id));
            graph.edges.push(EquationEdge { id: id.clone(), source: source.clone(), target: target.clone() });
            (vec![EquationMutation::ConnectNodes(ConnectNodes { id, source: source.clone(), target: target.clone(), index: None })], None)
        }
        EquationEditOperation::Disconnect { id } => {
            if !graph.edges.iter().any(|edge| &edge.id == id) {
                return (Vec::new(), None);
            }
            graph.edges.retain(|edge| &edge.id != id);
            (vec![EquationMutation::DisconnectNodes(DisconnectNodes { id: id.clone() })], None)
        }
        EquationEditOperation::Delete { node_ids, edge_ids } => {
            let removed: Vec<String> = graph.nodes.iter().filter(|node| node_ids.contains(&node.id)).map(|node| node.id.clone()).collect();
            let severed: Vec<String> = graph.edges.iter().filter(|edge| edge_ids.contains(&edge.id) || removed.contains(&edge.source) || removed.contains(&edge.target)).map(|edge| edge.id.clone()).collect();
            graph.edges.retain(|edge| !severed.contains(&edge.id));
            graph.nodes.retain(|node| !removed.contains(&node.id));
            let leaves = severed.into_iter().map(|id| EquationMutation::DisconnectNodes(DisconnectNodes { id })).chain(removed.into_iter().map(|id| EquationMutation::DeleteNode(DeleteNode { id }))).collect();
            (leaves, None)
        }
    }
}

/// 🛠️ The emit of a node-graph edit's leaves: a drag (a press named) commits as ONE tool transaction through the ONE
/// node-drag machine of `🛠️tool-machine` (design §13.3), the ref minted from the admission's authoring seed, the host clock
/// and `<appId>#nodeGraphEdit`; everything else, or a view without command authority, is one plain edit; nothing is the
/// empty emit (zero trace).
pub(crate) fn equation_edit_emit(authoring_seed: &str, gesture: Option<&str>, leaves: Vec<EquationMutation>) -> Emit<EquationMutation, NoConfigMutation> {
    let Some(gesture) = gesture else { return Emit::mutations(leaves) };
    node_drag_emit(EQUATION_EDITOR_APP_ID, NODE_GRAPH_EDIT_VERB, authoring_seed, gesture, leaves).into()
}

pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    let rows: Vec<JsonValue> = semio_framework_pack_json::parse(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_array().map(|values| values.to_vec())).unwrap_or_default();
    let mut graph = doc.snapshot.graph.clone();
    let (mut leaves, mut gesture) = (Vec::new(), None);
    for row in &rows {
        let (row_leaves, row_gesture) = equation_edit_operation_leaves(&mut graph, &EquationEditOperation::from_value(row)?);
        leaves.extend(row_leaves);
        gesture = gesture.or(row_gesture);
    }
    let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
    Ok(equation_edit_emit(authoring_seed, gesture.as_deref(), leaves))
}
