//! 🔺️ `change-node-label` — sparse diff construction: one node label patch.
use crate::diff::{EquationNodePatch, EquationNodesDelta};
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeLabel, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let Some(existing) = base.graph.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" already has label \"{}\".", payload.id, payload.new_label));
    }
    let patch = EquationNodePatch { id: payload.id.clone(), label: Some(payload.new_label.clone()), ..Default::default() };
    let diff = EquationDiff { nodes: Some(EquationNodesDelta { patched: vec![patch], ..Default::default() }), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base))
}
//#endregion 🔖️Diff
