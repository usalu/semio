//! 🔺️ `set-node-positions` — every addressed node lands at its payload position; a node the graph lacks is skipped
//! (`mutation.partial`).

use super::{equation_targets_invariant, SetNodePositions};
use crate::diff::{EquationNodePatch, EquationNodesDelta, EquationNodesModification};
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
    let nodes = &base.graph.nodes;
    let missing: Vec<String> = ids.iter().filter(|id| !nodes.iter().any(|node| &node.id == *id)).cloned().collect();
    if missing.len() == ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} node(s) exists.", ids.len()), missing);
    }
    let partial = (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} node(s) skipped (no such node): {}", missing.len(), ids.len(), missing.join(", "))).at(missing));
    let patched: Vec<EquationNodesModification> = payload
        .positions
        .iter()
        .filter(|position| nodes.iter().any(|node| node.id == position.id && (node.x, node.y) != (position.x, position.y)))
        .map(|position| EquationNodesModification { id: position.id.clone(), patch: EquationNodePatch { x: Some(position.x), y: Some(position.y), ..Default::default() } })
        .collect();
    if patched.is_empty() {
        return protocol::MutationOutcome::empty().absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "Every node already sits at its position.").at(ids)]));
    }
    let diff = EquationDiff { nodes: Some(EquationNodesDelta { modified: patched, ..Default::default() }), ..Default::default() };
    protocol::MutationOutcome::new(diff).absorb_messages(partial)
}
//#endregion 🔖️Diff
