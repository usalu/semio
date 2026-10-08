//! 🔺️ Sparse diff builder for `DeleteSupport`.
//!
//! 🛡️ No `mutation.target-referenced` guard: a support is a LEAF of the reference graph — it points
//! at a node and nothing in `Fem3dSnapshot` points back at it, so there is no referrer to protect.
//! Same for `delete-combination`.
use super::DeleteSupport;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dSupportRemoval, Fem3dSupportsDelta};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSupport, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(at) = base.supports.iter().position(|support| support.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Fem3dDiff { supports: Some(Fem3dSupportsDelta { removed: vec![Fem3dSupportRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
