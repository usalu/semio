//! 🔺️ Sparse diff builder for `ChangeLoad` — one patched load row inside the target case's keyed `loads` delta.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on `case_id`, the same code on
//! `load_id`, `mutation.target-mismatch` (Error) when the replacement renames the load, the SAME
//! per-variant target resolution `add-load` runs (`mutation.target-missing`, Error), the finite
//! magnitude bound (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ChangeLoad;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasePatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasesDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadCasesModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_mismatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::load_breach;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::resolve_load;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoad, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    let Some(held) = existing.loads.iter().find(|load| load_id(load) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load \"{}\" does not exist in case \"{}\".", payload.id, payload.case_id), [payload.id.clone()]);
    };

    let candidate = match payload.patch.candidate(held) { Ok(value) => value, Err(error) => return protocol::MutationOutcome::error("mutation.target-mismatch", error.message, [payload.case_id.clone(), payload.id.clone()]) };
    if load_id(&candidate) != payload.id {
        return target_mismatch("Load", &payload.id, load_id(&candidate));
    }
    if let Some(refusal) = resolve_load(base, &candidate) {
        return refusal;
    }
    if let Some(breach) = load_breach(&candidate) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    if *held == candidate {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Load \"{}\" in case \"{}\" already has that value.", payload.id, payload.case_id));
    }
    let loads = Fem3dLoadsDelta { modified: vec![Fem3dLoadsModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() };
    protocol::MutationOutcome::new(Fem3dDiff {
        load_cases: Some(Fem3dLoadCasesDelta { modified: vec![Fem3dLoadCasesModification { id: payload.case_id.clone(), patch: Fem3dLoadCasePatch { loads: Some(loads), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
