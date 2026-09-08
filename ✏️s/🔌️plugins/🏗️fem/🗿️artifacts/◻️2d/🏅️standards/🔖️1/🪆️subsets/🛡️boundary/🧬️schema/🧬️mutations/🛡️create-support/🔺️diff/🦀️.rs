//! 🔺️ Sparse diff builder for `CreateSupport`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::node_reference` resolution of `node_id` (`mutation.target-missing`, Error).
//! `replace-support` calls the SAME guard, so the twins cannot drift apart.
use super::CreateSupport;
use crate::diff::{Fem2dDiff, Fem2dSupportsDelta};
use crate::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSupport, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.supports.iter().any(|support| support.id == payload.support.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A support with id \"{}\" already exists.", payload.support.id), [payload.support.id.clone()]);
    }
    if let Some(rejection) = guards::node_reference(base, &payload.support.node_id) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { added: vec![payload.support.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
