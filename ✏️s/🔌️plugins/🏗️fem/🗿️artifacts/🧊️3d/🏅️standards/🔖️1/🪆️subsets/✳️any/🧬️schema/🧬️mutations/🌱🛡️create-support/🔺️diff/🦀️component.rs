//! 🔺️ Sparse diff builder for `CreateSupport`.
use super::mutation::CreateSupport;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSupportsDelta};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSupport, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.supports.iter().any(|support| support.id == payload.support.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A support with id \"{}\" already exists.", payload.support.id), [payload.support.id.clone()]);
    }
    if !base.nodes.iter().any(|node| node.id == payload.support.node_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.support.node_id), [payload.support.node_id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { added: vec![payload.support.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
