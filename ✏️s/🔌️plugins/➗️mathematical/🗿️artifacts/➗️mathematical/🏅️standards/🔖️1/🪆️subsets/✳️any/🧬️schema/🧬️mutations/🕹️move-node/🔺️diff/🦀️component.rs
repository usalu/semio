//! 🔺️ `move-node` — sparse diff construction.

use super::mutation::MoveNode;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MoveNode, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut graph = mathematical_graph(base);
    let Some(existing) = graph.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" position must be finite, got ({}, {}).", payload.id, payload.x, payload.y), [payload.id.clone()]);
    }
    if existing.x == payload.x && existing.y == payload.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" is already at ({}, {}).", payload.id, payload.x, payload.y));
    }
    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.x = payload.x;
        node.y = payload.y;
    }
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
