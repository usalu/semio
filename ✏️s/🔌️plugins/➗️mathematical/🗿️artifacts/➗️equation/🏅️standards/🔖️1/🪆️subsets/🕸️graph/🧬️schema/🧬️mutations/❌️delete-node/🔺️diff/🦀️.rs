//! 🔺️ `delete-node` — sparse diff construction: the removed node id and every incident edge id (the cascade).
use crate::diff::{EquationEdgesDelta, EquationNodesDelta};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteNode, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let Some(node_index) = base.graph.nodes.iter().position(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let cascaded_edge_ids: Vec<String> = base.graph.edges.iter().filter(|edge| edge.source == payload.id || edge.target == payload.id).map(|edge| edge.id.clone()).collect();
    let edge_indices: Vec<usize> = base.graph.edges.iter().enumerate().filter(|(_, edge)| edge.source == payload.id || edge.target == payload.id).map(|(index, _)| index).collect();
    let diff = EquationDiff {
        nodes: Some(EquationNodesDelta::removal(&base.graph.nodes, node_index)),
        edges: (!edge_indices.is_empty()).then(|| EquationEdgesDelta::removals(&base.graph.edges, &edge_indices)),
        ..Default::default()
    };
    let outcome = protocol::MutationOutcome::new(diff);
    if cascaded_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting node \"{}\" also removed {} connected edge(s): {}.", payload.id, cascaded_edge_ids.len(), cascaded_edge_ids.join(", ")))
    }
}
//#endregion 🔖️Diff
