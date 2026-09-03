//! 🔺️ `delete-node` — sparse diff construction, cascading to incident edges.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::DeleteNode, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = equation_graph(base);
    if !graph.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let cascaded_edge_ids: Vec<String> = graph.edges.iter().filter(|edge| edge.source == payload.id || edge.target == payload.id).map(|edge| edge.id.clone()).collect();
    graph.nodes.retain(|node| node.id != payload.id);
    graph.edges.retain(|edge| edge.source != payload.id && edge.target != payload.id);
    let (notation, results, computed) = equation_children_from_state(&graph, &equation_geometry(base));
    let outcome = protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() });
    if cascaded_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting node \"{}\" also removed {} connected edge(s): {}.", payload.id, cascaded_edge_ids.len(), cascaded_edge_ids.join(", ")))
    }
}
//#endregion 🔖️Diff
