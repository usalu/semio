//! 🔺️ `create-node` — sparse diff construction.

use crate::{EquationDiff, EquationNode, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is Fatal `duplicate-id` — an id-keyed entity that already exists cannot be
/// "created" again.
pub fn diff(payload: &super::CreateNode, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = base.graph.clone();
    if graph.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    graph.nodes.insert(payload.index.map_or(graph.nodes.len(), |index| index.min(graph.nodes.len())), EquationNode { id: payload.id.clone(), label: payload.label.clone(), x: payload.x, y: payload.y });
    protocol::MutationOutcome::new(crate::equation_state_diff(graph, base.geometry.clone()))
}
//#endregion 🔖️Diff
