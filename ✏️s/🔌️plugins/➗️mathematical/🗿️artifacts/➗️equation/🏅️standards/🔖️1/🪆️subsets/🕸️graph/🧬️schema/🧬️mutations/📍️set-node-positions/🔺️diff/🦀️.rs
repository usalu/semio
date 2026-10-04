//! 🔺️ `set-node-positions` — every addressed node lands at its payload position; a node the graph lacks is skipped
//! (`mutation.partial`).

use super::{equation_targets_invariant, SetNodePositions};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
/// 🏗️ A malformed position list is `mutation.invariant`; none left to place is `mutation.target-missing`; every node
/// already in place is `mutation.no-op`.
pub fn diff(payload: &SetNodePositions, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let ids = payload.ids();
    if let Err(reason) = equation_targets_invariant(&ids) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, ids);
    }
    if payload.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A node position must be finite.", ids);
    }
    let mut graph = base.graph.clone();
    let missing: Vec<String> = ids.iter().filter(|id| !graph.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
    if missing.len() == ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} node(s) exists.", ids.len()), missing);
    }
    let partial = (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} node(s) skipped (no such node): {}", missing.len(), ids.len(), missing.join(", "))).at(missing));
    let mut changed = false;
    for position in &payload.positions {
        if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == position.id) {
            changed |= (node.x, node.y) != (position.x, position.y);
            node.x = position.x;
            node.y = position.y;
        }
    }
    if !changed {
        return protocol::MutationOutcome::empty().absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "Every node already sits at its position.").at(ids)]));
    }
    protocol::MutationOutcome::new(crate::equation_state_diff(graph, base.geometry.clone())).absorb_messages(partial)
}
//#endregion 🔖️Diff
