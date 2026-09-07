//! 🔺️ Sparse diff builder for `DeleteSupport`.
//!
//! One guard only: `mutation.target-missing` (Error). No referential guard is possible or needed —
//! a support id is named by no other record in the vocabulary, so removing one can never orphan a
//! reference. It is the only `delete-` besides `delete-node` that carries no
//! `mutation.target-referenced` branch, and for a structural rather than a specified reason.
use super::DeleteSupport;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dSupportsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSupport, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.supports.iter().any(|support| support.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { supports: Some(Fem2dSupportsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
