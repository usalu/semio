//! 🔺️ Sparse diff builder for `CreateCombination`.
use super::CreateCombination;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dCombinationInsertion, Fem3dCombinationsDelta, Fem3dDiff};
use crate::standards::v1::subsets::any::schema::mutations::{combination_breach,invariant,resolve_combination_terms};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateCombination, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.combinations.iter().any(|combination| combination.id == payload.combination.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A combination with id \"{}\" already exists.", payload.combination.id), [payload.combination.id.clone()]);
    }
    if let Some(refusal) = resolve_combination_terms(base, &payload.combination) {
        return refusal;
    }
    if let Some(breach) = combination_breach(&payload.combination) {
        return invariant(breach, vec![payload.combination.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.combinations.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.combinations.len()), [&payload.combination.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { inserted: vec![Fem3dCombinationInsertion { index: payload.index.unwrap_or(base.combinations.len()), row: payload.combination.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
