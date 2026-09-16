//! 🔺️ Sparse diff builder for `ReplaceCombination`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME per-term resolution
//! `create-combination` runs (`mutation.target-missing` on a case this base does not carry), the
//! finite-factor bound (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceCombination;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dCombinationsDelta, Fem3dCombinationsPatchEntry, Fem3dDiff};
use crate::standards::v1::subsets::any::schema::mutations::{combination_breach, id_mismatch, invariant, resolve_combination_terms};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceCombination, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.combinations.iter().find(|combination| combination.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Combination \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_combination.id != payload.id {
        return id_mismatch("Combination", &payload.id, &payload.new_combination.id);
    }
    if let Some(refusal) = resolve_combination_terms(base, &payload.new_combination) {
        return refusal;
    }
    if let Some(breach) = combination_breach(&payload.new_combination) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    if *existing == payload.new_combination {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Combination \"{}\" already has that value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { combinations: Some(Fem3dCombinationsDelta { patched: vec![Fem3dCombinationsPatchEntry { id: payload.id.clone(), item: payload.new_combination.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
