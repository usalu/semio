//! 🔺️ `delete-node` — sparse diff construction: the removed node id and every incident edge id (the cascade).
use crate::diff::{EquationEdgesDelta, EquationNodesDelta};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteNode, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if !base.graph.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let cascaded_edge_ids: Vec<String> = base.graph.edges.iter().filter(|edge| edge.source == payload.id || edge.target == payload.id).map(|edge| edge.id.clone()).collect();
    let diff = EquationDiff {
        nodes: Some(EquationNodesDelta { removed: vec![payload.id.clone()], ..Default::default() }),
        edges: (!cascaded_edge_ids.is_empty()).then(|| EquationEdgesDelta { removed: cascaded_edge_ids.clone(), ..Default::default() }),
        ..Default::default()
    };
    let outcome = protocol::MutationOutcome::new(crate::equation_state_diff(diff, base));
    if cascaded_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting node \"{}\" also removed {} connected edge(s): {}.", payload.id, cascaded_edge_ids.len(), cascaded_edge_ids.join(", ")))
    }
}
//#endregion 🔖️Diff
