//! 🔺️ `move-nodes` — every addressed node moves by the payload offset from its BASE position; a node the graph lacks is
//! skipped (`mutation.partial`).

use crate::standards::v1::subsets::graph::schema::mutations::set_node_positions::equation_targets_invariant;
use crate::diff::{EquationNodePatch, EquationNodesDelta};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
/// 🏗️ A malformed target list or a non-finite offset is `mutation.invariant`; none left to move is
/// `mutation.target-missing`; a zero offset is `mutation.no-op`.
pub fn diff(payload: &super::MoveNodes, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if let Err(reason) = equation_targets_invariant(&payload.ids) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, payload.ids.clone());
    }
    if !payload.dx.is_finite() || !payload.dy.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a node offset must be finite", payload.ids.clone());
    }
    let nodes = &base.graph.nodes;
    let missing: Vec<String> = payload.ids.iter().filter(|id| !nodes.iter().any(|node| &node.id == *id)).cloned().collect();
    if missing.len() == payload.ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} node(s) exists.", payload.ids.len()), missing);
    }
    let partial = (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} node(s) skipped (no such node): {}", missing.len(), payload.ids.len(), missing.join(", "))).at(missing));
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::empty().absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "The drag offset is zero.").at(payload.ids.clone())]));
    }
    let patched: Vec<EquationNodePatch> = nodes.iter().filter(|node| payload.ids.contains(&node.id)).map(|node| EquationNodePatch { id: node.id.clone(), x: Some(node.x + payload.dx), y: Some(node.y + payload.dy), ..Default::default() }).collect();
    if patched.iter().any(|patch| patch.x.is_some_and(|x| !x.is_finite()) || patch.y.is_some_and(|y| !y.is_finite())) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", "The moved position leaves the finite canvas.", payload.ids.clone());
    }
    let diff = EquationDiff { nodes: Some(EquationNodesDelta { patched, ..Default::default() }), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base)).absorb_messages(partial)
}
//#endregion 🔖️Diff
