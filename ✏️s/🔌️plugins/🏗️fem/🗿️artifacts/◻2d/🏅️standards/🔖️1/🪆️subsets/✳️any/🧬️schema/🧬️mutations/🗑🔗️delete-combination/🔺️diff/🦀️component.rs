//! 🔺️ Sparse diff builder for `DeleteCombination`.
use super::mutation::DeleteCombination;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dCombinationsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteCombination, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.combinations.iter().any(|combination| combination.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Combination \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { combinations: Some(Fem2dCombinationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
