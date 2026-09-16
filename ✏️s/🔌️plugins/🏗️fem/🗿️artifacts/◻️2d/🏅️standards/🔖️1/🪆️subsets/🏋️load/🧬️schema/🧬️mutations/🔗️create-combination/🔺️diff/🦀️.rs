//! 🔺️ Sparse diff builder for `CreateCombination`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the SAME per-term
//! resolution `replace-combination` runs (`mutation.target-missing` on a term this base cannot
//! resolve, `mutation.invariant` on a term weighting the combination itself) and the same
//! finite-factor bound (`mutation.invariant`, Fatal).
use super::CreateCombination;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dCombinationsDelta, Fem2dDiff};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateCombination, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.combinations.iter().any(|combination| combination.id == payload.combination.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A combination with id \"{}\" already exists.", payload.combination.id), [payload.combination.id.clone()]);
    }
    if let Some(rejection) = guards::combination_term_references(base, &payload.combination.id, &payload.combination) {
        return rejection;
    }
    if let Some(rejection) = guards::combination_factors(&payload.combination) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { combinations: Some(Fem2dCombinationsDelta { added: vec![payload.combination.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
