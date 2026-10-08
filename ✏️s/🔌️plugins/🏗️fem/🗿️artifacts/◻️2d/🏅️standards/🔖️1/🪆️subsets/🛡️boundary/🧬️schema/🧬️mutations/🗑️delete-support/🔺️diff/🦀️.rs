//! 🔺️ Sparse diff builder for `DeleteSupport`.
//!
//! One guard only: `mutation.target-missing` (Error). No referential guard is possible or needed —
//! a support id is named by no other record in the vocabulary, so removing one can never orphan a
//! reference. It is the only `delete-` besides `delete-node` that carries no
//! `mutation.target-referenced` branch, and for a structural rather than a specified reason.
use super::DeleteSupport;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dSupportRemoval, Fem2dSupportsDelta};
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSupport, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(at) = base.supports.iter().position(|support| support.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { removed: vec![Fem2dSupportRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
