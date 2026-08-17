//! 🔺️ Sparse diff builder for `CreateNode`.
use super::mutation::CreateNode;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dNodesDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateNode, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { nodes: Some(Fem3dNodesDelta { added: vec![payload.node.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
