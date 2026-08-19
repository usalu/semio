//! 🔺️ Sparse diff builder for `CreateCombination`.
use super::mutation::CreateCombination;
use crate::artifacts::fem3d::diff::{Fem3dCombinationsDelta, Fem3dDiff};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateCombination, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.combinations.iter().any(|combination| combination.id == payload.combination.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A combination with id \"{}\" already exists.", payload.combination.id), [payload.combination.id.clone()]);
    }
    for case_id in payload.combination.terms.keys() {
        if !base.load_cases.iter().any(|case| &case.id == case_id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", case_id), [case_id.clone()]);
        }
    }
    protocol::MutationOutcome::new(Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { added: vec![payload.combination.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
