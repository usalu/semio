//! 🔺️ Sparse diff builder for `DeleteCombination`.
//!
//! 🔗️ No `mutation.target-referenced` guard: a combination is a LEAF of the reference graph — it
//! weights load cases and nothing in `Fem3dSnapshot` points back at it. Same for `delete-support`.
use super::DeleteCombination;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dCombinationsDelta, Fem3dDiff, Fem3dCombinationRemoval};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteCombination, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(at) = base.combinations.iter().position(|combination| combination.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Combination \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { removed: vec![Fem3dCombinationRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
