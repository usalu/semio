//! 🔺️ Sparse diff builder for `ChangeLoad` — one patched load row inside the target case's keyed `loads` delta.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on `case_id`, the same code on
//! `load_id`, `mutation.target-mismatch` (Error) when the replacement renames the load,
//! the SAME per-variant target resolution `add-load` runs (`mutation.target-missing`, Error), the
//! finite-magnitude bound (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ChangeLoad;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCasePatch;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCasesDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadCasesModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dLoadsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoad, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    let Some(held) = existing.loads.iter().find(|load| load_id(load) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load \"{}\" does not exist in case \"{}\".", payload.id, payload.case_id), [payload.id.clone()]);
    };

    let candidate = match payload.patch.candidate(held) { Ok(value) => value, Err(error) => return protocol::MutationOutcome::error("mutation.target-mismatch", error.message, [payload.case_id.clone(), payload.id.clone()]) };
    if let Some(rejection) = guards::load_reference(base, &candidate) {
        return rejection;
    }
    if let Some(rejection) = guards::load_magnitudes(&candidate) {
        return rejection;
    }
    if *held == candidate {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Load \"{}\" in case \"{}\" is already equal to the replacement value.", payload.id, payload.case_id));
    }
    let loads = Fem2dLoadsDelta { modified: vec![Fem2dLoadsModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() };
    protocol::MutationOutcome::new(Fem2dDiff {
        load_cases: Some(Fem2dLoadCasesDelta { modified: vec![Fem2dLoadCasesModification { id: payload.case_id.clone(), patch: Fem2dLoadCasePatch { loads: Some(loads), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
