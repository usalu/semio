//! 🔺️ `move-node` — sparse diff construction.

use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::MoveNode, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = base.graph.clone();
    let Some(existing) = graph.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" position must be finite, got ({}, {}).", payload.id, payload.x, payload.y), [payload.id.clone()]);
    }
    if existing.x == payload.x && existing.y == payload.y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" is already at ({}, {}).", payload.id, payload.x, payload.y));
    }
    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.x = payload.x;
        node.y = payload.y;
    }
    protocol::MutationOutcome::new(crate::equation_state_diff(graph, base.geometry.clone()))
}
//#endregion 🔖️Diff
