//! 🔺️ Sparse diff builder for `CreateSupport`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::node_reference` resolution of `node_id` (`mutation.target-missing`, Error).
//! `replace-support` calls the SAME guard, so the twins cannot drift apart.
use super::CreateSupport;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dSupportInsertion, Fem2dSupportsDelta};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSupport, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.supports.iter().any(|support| support.id == payload.support.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A support with id \"{}\" already exists.", payload.support.id), [payload.support.id.clone()]);
    }
    if let Some(rejection) = guards::node_reference(base, &payload.support.node_id) {
        return rejection;
    }
    if payload.index.is_some_and(|at| at > base.supports.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.supports.len()), [&payload.support.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { inserted: vec![Fem2dSupportInsertion { index: payload.index.unwrap_or(base.supports.len()), row: payload.support.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
