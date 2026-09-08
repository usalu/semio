//! 🔺️ Sparse diff builder for `DeleteCombination`.
//!
//! 🔗️ No `mutation.target-referenced` guard: a combination is a LEAF of the reference graph — it
//! weights load cases and nothing in `Fem3dSnapshot` points back at it. Same for `delete-support`.
use super::DeleteCombination;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dCombinationsDelta, Fem3dDiff};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteCombination, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.combinations.iter().any(|combination| combination.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Combination \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
