//! 🔺️ `change-node-label` — sparse diff construction.

use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeLabel, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = base.graph.clone();
    let Some(existing) = graph.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" already has label \"{}\".", payload.id, payload.new_label));
    }
    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.label = payload.new_label.clone();
    }
    protocol::MutationOutcome::new(crate::equation_state_diff(graph, base.geometry.clone()))
}
//#endregion 🔖️Diff
