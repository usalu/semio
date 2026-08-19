//! 🔺️ Sparse diff builder for `DeleteCombination`.
use super::mutation::DeleteCombination;
use crate::artifacts::fem3d::diff::{Fem3dCombinationsDelta, Fem3dDiff};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteCombination, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.combinations.iter().any(|combination| combination.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Combination \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
