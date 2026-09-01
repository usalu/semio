//! 🔺️ Sparse diff builder for `CreateNode`.
use super::CreateNode;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dNodesDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { nodes: Some(Fem2dNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
