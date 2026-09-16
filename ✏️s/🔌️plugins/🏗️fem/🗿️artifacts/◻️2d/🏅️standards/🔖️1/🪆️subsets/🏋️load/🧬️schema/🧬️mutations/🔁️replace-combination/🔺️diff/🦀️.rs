//! 🔺️ Sparse diff builder for `ReplaceCombination`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME per-term resolution
//! `create-combination` runs (`mutation.target-missing` on a term this base cannot resolve,
//! `mutation.invariant` on a term weighting the combination itself), the finite-factor bound
//! (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceCombination;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dCombinationsDelta, Fem2dCombinationsPatchEntry, Fem2dDiff};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceCombination, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.combinations.iter().find(|combination| combination.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Combination \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("combination", &payload.id, &payload.new_combination.id) {
        return rejection;
    }
    if let Some(rejection) = guards::combination_term_references(base, &payload.id, &payload.new_combination) {
        return rejection;
    }
    if let Some(rejection) = guards::combination_factors(&payload.new_combination) {
        return rejection;
    }
    if *existing == payload.new_combination {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Combination \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { combinations: Some(Fem2dCombinationsDelta { patched: vec![Fem2dCombinationsPatchEntry { id: payload.id.clone(), item: payload.new_combination.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
