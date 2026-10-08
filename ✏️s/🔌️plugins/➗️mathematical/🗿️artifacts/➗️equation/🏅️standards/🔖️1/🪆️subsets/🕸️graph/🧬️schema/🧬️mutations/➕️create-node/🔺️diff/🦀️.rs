//! 🔺️ `create-node` — sparse diff construction: one added node (and the order when it is not appended).
use crate::diff::EquationNodesDelta;
use crate::{EquationDiff, EquationNode, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is Fatal `duplicate-id` — an id-keyed entity that already exists cannot be
/// "created" again.
pub fn diff(payload: &super::CreateNode, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let nodes = &base.graph.nodes;
    if nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let at = payload.index.map_or(nodes.len(), |index| index.min(nodes.len()));
    let node = EquationNode { id: payload.id.clone(), label: payload.label.clone(), x: payload.x, y: payload.y };
    let diff = EquationDiff { nodes: Some(EquationNodesDelta::insertion(at, node)), ..Default::default() };
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
