//! 🔺️ Sparse diff builder for `CreateSupport`.
use super::mutation::CreateSupport;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSupportsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSupport, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.supports.iter().any(|support| support.id == payload.support.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A support with id \"{}\" already exists.", payload.support.id), [payload.support.id.clone()]);
    }
    if !base.nodes.iter().any(|node| node.id == payload.support.node_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.support.node_id), [payload.support.node_id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { added: vec![payload.support.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
