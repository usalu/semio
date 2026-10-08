//! 🔺️ Sparse diff builder for `CreateSupport`.
use super::CreateSupport;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dSupportInsertion, Fem3dSupportsDelta};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSupport, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.supports.iter().any(|support| support.id == payload.support.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A support with id \"{}\" already exists.", payload.support.id), [payload.support.id.clone()]);
    }
    if !base.nodes.iter().any(|node| node.id == payload.support.node_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.support.node_id), [payload.support.node_id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.supports.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.supports.len()), [&payload.support.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { inserted: vec![Fem3dSupportInsertion { index: payload.index.unwrap_or(base.supports.len()), row: payload.support.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
