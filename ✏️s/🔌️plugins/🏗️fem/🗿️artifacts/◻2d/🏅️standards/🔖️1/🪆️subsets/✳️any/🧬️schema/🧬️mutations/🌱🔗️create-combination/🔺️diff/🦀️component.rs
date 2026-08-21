//! 🔺️ Sparse diff builder for `CreateCombination`.
use super::mutation::CreateCombination;
use crate::artifacts::fem2d::diff::{Fem2dCombinationsDelta, Fem2dDiff};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &CreateCombination, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.combinations.iter().any(|combination| combination.id == payload.combination.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A combination with id \"{}\" already exists.", payload.combination.id), [payload.combination.id.clone()]);
    }
    for term in &payload.combination.terms {
        let referenced_exists = base.load_cases.iter().any(|case| case.id == term.case_id) || base.combinations.iter().any(|combination| combination.id == term.case_id);
        if !referenced_exists {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case or combination \"{}\" does not exist.", term.case_id), [term.case_id.clone()]);
        }
    }
    protocol::MutationOutcome::new(Fem2dDiff { combinations: Some(Fem2dCombinationsDelta { added: vec![payload.combination.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
