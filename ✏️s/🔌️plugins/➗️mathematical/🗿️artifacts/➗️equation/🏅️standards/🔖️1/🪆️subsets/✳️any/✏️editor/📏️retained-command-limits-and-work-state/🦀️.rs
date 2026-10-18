//! 📏️ Retained command limits and work state: the equation tool ids and retained capacities, the publication contracts, the identities, the graph shape and edit preflight, the command extent, the work phases and close units and the retained command work struct. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
pub const EQUATION_TOOL_IDS: &[&str] = &["editEquation", "setAlgorithm", "setDirected", "nodeGraphEdit", "nodeGraphViewport", "editPoints", "setActiveExample", "addNode"];

pub(super) const EQUATION_RETAINED_PAYLOAD_SCHEMA: &str = "semio.equation/v1.tool-command.v1";

pub(super) const EQUATION_RETAINED_RAW_BYTES: usize = 65_536;

pub(super) const EQUATION_RETAINED_WORK_ITEMS: usize = 65_536;

pub const EQUATION_MAX_NODES: usize = 256;

pub const EQUATION_MAX_EDGES: usize = 512;

pub const EQUATION_MAX_POINTS: usize = 1_024;

pub const EQUATION_MAX_EDIT_JSON_BYTES: usize = 8_192;

pub const EQUATION_MAX_EDIT_OPERATIONS: usize = 16;

pub(crate) const EQUATION_MAX_DELETE_IDS: usize = 256;

pub(crate) const EQUATION_MAX_TEXT_BYTES: usize = 256;

/// 🧬️ The `nodeGraphEdit` argument the Actions pane stages by default — one node-graph gesture record `move` of the demo's
/// node `a`, in the exact shared row shape `EquationEditOperation::from_value` admits.
pub const EQUATION_DEFAULT_EDIT_OPERATIONS: &str = r#"[{"operation":"move","gestureId":"actions-pane","nodeIds":["a"],"dx":40.0,"dy":20.0}]"#;

pub(super) const EQUATION_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "editEquation", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAlgorithm", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setDirected", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "editPoints", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

pub(super) fn equation_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(EQUATION_RETAINED_RAW_BYTES, 2_048, 1, EQUATION_RETAINED_WORK_ITEMS, 7_500, 1, 1)
}

fn equation_tool_identity(tool_id: &str) -> u64 {
    tool_id.bytes().fold(0xcbf2_9ce4_8422_2325, |digest, byte| (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3))
}

pub fn equation_operation_identity(tool_id: &str, operation: &AppOperationContext) -> u64 {
    let mut identity = equation_tool_identity(tool_id);
    let app_instance = operation.app_instance_id.to_le_bytes();
    let operation_id = operation.operation_id.to_le_bytes();
    let generation = operation.generation.to_le_bytes();
    for bytes in [app_instance.as_slice(), operation.parent_document_id.as_bytes(), operation_id.as_slice(), generation.as_slice(), operation.canonical_base_revision.as_slice()] {
        for byte in bytes {
            identity = (identity ^ u64::from(*byte)).wrapping_mul(0x1000_0000_01b3);
        }
    }
    identity
}

fn equation_graph_shape_admitted(graph: &EquationGraph) -> bool {
    graph.nodes.len() <= EQUATION_MAX_NODES && graph.edges.len() <= EQUATION_MAX_EDGES && graph.algorithm.len() <= EQUATION_MAX_TEXT_BYTES && graph.algorithm_seed.as_ref().is_none_or(|seed| seed.len() <= EQUATION_MAX_TEXT_BYTES)
}

pub fn equation_edit_preflight(payload: &node_graph_edit::NodeGraphEdit) -> Option<usize> {
    if payload.operations_json.len() > EQUATION_MAX_EDIT_JSON_BYTES {
        return None;
    }
    let values = semio_framework_pack_json::parse(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_array().map(|values| values.to_vec()))?;
    if values.len() > EQUATION_MAX_EDIT_OPERATIONS {
        return None;
    }
    for value in &values {
        EquationEditOperation::from_value(value).ok()?;
    }
    Some(values.len())
}

pub fn equation_command_extent(command: &EquationCommand, snapshot: &EquationSnapshot) -> Option<usize> {
    if let EquationCommand::SetActiveExample(payload) = command {
        return (payload.example_id.len() <= EQUATION_MAX_TEXT_BYTES).then_some(1);
    }
    if !equation_graph_shape_admitted(&snapshot.graph) || snapshot.geometry.points.len() > EQUATION_MAX_POINTS {
        return None;
    }
    let extent = match command {
        EquationCommand::SetActiveExample(_) => return None,
        EquationCommand::NodeGraphViewport(_) => 1,
        // 🧮️ `EquationRetainedCommandWork::step` charges ONE step per phase BOUNDARY on top of the
        // per-item steps: `Initialize`, then `nodes-complete`, then `edges-complete` (`Finish` itself
        // completes without a step). A graph-walking verb therefore costs `3 + nodes + edges`, not
        // `2 + …` — the old constant priced exactly one boundary (the shape `SetPoints` has) and every
        // `setAlgorithm`/`setDirected` overflowed its own extent by one step with
        // `equation-work-extent-overflow` on the very last boundary.
        EquationCommand::SetAlgorithm(payload) if payload.algorithm.len() <= EQUATION_MAX_TEXT_BYTES && payload.seed.as_ref().is_none_or(|seed| seed.len() <= EQUATION_MAX_TEXT_BYTES) => 1,
        EquationCommand::SetAlgorithm(_) => return None,
        EquationCommand::SetDirected(_) => 1,
        EquationCommand::AddNode(payload) if payload.x.is_finite() && payload.y.is_finite() => 1,
        EquationCommand::AddNode(_) => return None,
        EquationCommand::EditPoints(payload) if payload.geometry.points.len() <= EQUATION_MAX_POINTS => 2_usize.checked_add(payload.geometry.points.len())?,
        EquationCommand::EditPoints(_) => return None,
        EquationCommand::EditEquation(payload)
            if payload.graph.retained_node_count() <= EQUATION_MAX_NODES
                && payload.graph.retained_edge_count() <= EQUATION_MAX_EDGES
                && payload.geometry.points.len() <= EQUATION_MAX_POINTS
                && payload.graph.retained_metadata().1.len() <= EQUATION_MAX_TEXT_BYTES
                && payload.graph.retained_metadata().2.is_none_or(|seed| seed.len() <= EQUATION_MAX_TEXT_BYTES) =>
        {
            // 🗿️ `setArtifact` walks THREE phases (nodes, edges, points), so it pays `Initialize` plus
            // three boundaries — see the `setAlgorithm` note above.
            4_usize.checked_add(payload.graph.retained_node_count())?.checked_add(payload.graph.retained_edge_count())?.checked_add(payload.geometry.points.len())?
        }
        EquationCommand::EditEquation(_) => return None,
        EquationCommand::NodeGraphEdit(payload) => 4_usize.checked_add(payload.operations_json.len())?.checked_add(equation_edit_preflight(payload)?)?,
    };
    (extent != 0 && extent <= EQUATION_RETAINED_WORK_ITEMS).then_some(extent)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EquationWorkPhase {
    Initialize,
    Nodes,
    Edges,
    Points,
    JsonBytes,
    JsonDecode,
    Operations,
    Finish,
}

/// 📏️ The heap one yielded leaf retains while a retained job holds it.
pub(super) fn equation_leaf_retained_bytes(leaf: &EquationMutation) -> usize {
    use crate::standards::v1::subsets::graph::schema::mutations::{connect_nodes::ConnectNodes, create_node::CreateNode, delete_node::DeleteNode, disconnect_nodes::DisconnectNodes, move_nodes::MoveNodes, update_graph_algorithm::UpdateGraphAlgorithm};
    size_of::<EquationMutation>()
        + match leaf {
            EquationMutation::CreateNode(CreateNode { id, label, .. }) => id.capacity() + label.capacity(),
            EquationMutation::MoveNodes(MoveNodes { ids, .. }) => ids.capacity() * size_of::<String>() + ids.iter().map(String::capacity).sum::<usize>(),
            EquationMutation::ConnectNodes(ConnectNodes { id, source, target, .. }) => id.capacity() + source.capacity() + target.capacity(),
            EquationMutation::DisconnectNodes(DisconnectNodes { id }) | EquationMutation::DeleteNode(DeleteNode { id }) => id.capacity(),
            EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm, new_algorithm_seed }) => new_algorithm.capacity() + new_algorithm_seed.as_ref().map_or(0, String::capacity),
            _ => 0,
        }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EquationCloseUnit {
    Operation,
    OperationsBacking,
    Leaf,
    LeavesBacking,
    Gesture,
    Point,
    PointsBacking,
    Edge,
    EdgesBacking,
    Node,
    NodesBacking,
    Graph,
}

pub struct EquationRetainedCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) operation_identity: u64,
    pub(super) extent: usize,
    pub(super) phase: EquationWorkPhase,
    pub(super) item_cursor: usize,
    pub(super) cursor: usize,
    pub(super) digest: u64,
    pub(super) replay_target: Option<(usize, u64)>,
    pub(super) graph: Option<EquationGraph>,
    pub(super) points: Vec<crate::EquationPoint>,
    pub(super) operations: Vec<EquationEditOperation>,
    pub(super) leaves: Vec<EquationMutation>,
    pub(super) gesture: Option<String>,
    pub(super) closing: bool,
}
//#endregion 🧵️RetainedCommands
