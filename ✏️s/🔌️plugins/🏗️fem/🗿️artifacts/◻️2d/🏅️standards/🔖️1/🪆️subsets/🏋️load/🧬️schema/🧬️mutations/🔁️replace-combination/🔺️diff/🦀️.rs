//! 🔺️ Sparse diff builder for `ReplaceCombination`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.target-mismatch` (Error) when the replacement renames it, the SAME per-term resolution
//! `create-combination` runs (`mutation.target-missing` on a term this base cannot resolve,
//! `mutation.invariant` on a term weighting the combination itself), the finite-factor bound
//! (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceCombination;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dCombinationsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dCombinationsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceCombination, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
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
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Combination \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { combinations: Some(Fem2dCombinationsDelta { modified: vec![Fem2dCombinationsModification { id: payload.id.clone(), patch: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dCombinationPatch::replacement(existing, &payload.new_combination) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
