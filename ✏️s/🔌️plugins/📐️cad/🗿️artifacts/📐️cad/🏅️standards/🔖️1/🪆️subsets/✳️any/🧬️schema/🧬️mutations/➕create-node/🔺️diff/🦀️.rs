//! 🔺️ Sparse diff builder for `CreateNode`.
use super::CreateNode;
use crate::diff::{CadDiff, CadNodesDelta};
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    let index = payload.index.map_or(base.nodes.len(), |index| index as usize);
    if index > base.nodes.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node insertion index {index} exceeds the {} existing nodes.", base.nodes.len()), [payload.node.id.clone()]);
    }
    let reordered = (index != base.nodes.len()).then(|| {
        let mut order: Vec<String> = base.nodes.iter().map(|node| node.id.clone()).collect();
        order.insert(index, payload.node.id.clone());
        order
    });
    protocol::MutationOutcome::new(CadDiff { nodes: Some(CadNodesDelta { added: vec![payload.node.clone()], reordered, ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
