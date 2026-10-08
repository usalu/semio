//! 🔺️ `move-node` — sparse diff construction: one node position patch.
use crate::diff::{EquationNodePatch, EquationNodesDelta};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::MoveNode, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let Some(existing) = base.graph.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" position must be finite, got ({}, {}).", payload.id, payload.x, payload.y), [payload.id.clone()]);
    }
    if existing.x == payload.x && existing.y == payload.y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" is already at ({}, {}).", payload.id, payload.x, payload.y));
    }
    let patch = EquationNodePatch { id: payload.id.clone(), x: Some(payload.x), y: Some(payload.y), ..Default::default() };
    let diff = EquationDiff { nodes: Some(EquationNodesDelta { patched: vec![patch], ..Default::default() }), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base))
}
//#endregion 🔖️Diff
