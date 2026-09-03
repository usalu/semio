//! 🔺️ `connect-nodes` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationEdge, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate edge `id` is Fatal `duplicate-id`, matching `create-node`'s handling. A missing
/// endpoint node is Error `target-missing`. A parallel edge (same source/target as an existing
/// edge, under a fresh id) is Warning `no-op` — parallel edges are forbidden in this graph model.
pub async fn diff(payload: &super::ConnectNodes, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = equation_graph(base);
    if graph.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let missing: Vec<String> = [&payload.source, &payload.target].into_iter().filter(|id| !graph.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
    if !missing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node(s) {} do not exist.", missing.join(", ")), missing);
    }
    if graph.edges.iter().any(|edge| edge.source == payload.source && edge.target == payload.target) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("An edge from \"{}\" to \"{}\" already exists; parallel edges are not allowed.", payload.source, payload.target));
    }
    graph.edges.push(EquationEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone() });
    let (notation, results, computed) = equation_children_from_state(&graph, &equation_geometry(base));
    protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
