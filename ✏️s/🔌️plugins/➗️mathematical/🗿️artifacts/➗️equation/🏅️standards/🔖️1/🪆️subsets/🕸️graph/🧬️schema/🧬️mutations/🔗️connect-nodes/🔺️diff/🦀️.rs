//! 🔺️ `connect-nodes` — sparse diff construction: one added edge (and the order when it is not appended).
use crate::diff::EquationEdgesDelta;
use crate::{EquationDiff, EquationEdge, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate edge `id` is Fatal `duplicate-id`, matching `create-node`'s handling. A missing
/// endpoint node is Error `target-missing`. A parallel edge (same source/target as an existing
/// edge, under a fresh id) is Warning `no-op` — parallel edges are forbidden in this graph model.
pub fn diff(payload: &super::ConnectNodes, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let graph = &base.graph;
    if graph.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let missing: Vec<String> = [&payload.source, &payload.target].into_iter().filter(|id| !graph.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
    if !missing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node(s) {} do not exist.", missing.join(", ")), missing);
    }
    if graph.edges.iter().any(|edge| edge.source == payload.source && edge.target == payload.target) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("An edge from \"{}\" to \"{}\" already exists; parallel edges are not allowed.", payload.source, payload.target));
    }
    let reordered = payload.index.filter(|index| *index < graph.edges.len()).map(|index| {
        let mut order: Vec<String> = graph.edges.iter().map(|edge| edge.id.clone()).collect();
        order.insert(index, payload.id.clone());
        order
    });
    let edge = EquationEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone() };
    let diff = EquationDiff { edges: Some(EquationEdgesDelta { added: vec![edge], reordered, ..Default::default() }), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base))
}
//#endregion 🔖️Diff
