//! 🔺️ Sparse diff builder for `ReplaceLoad` — one patched load row inside the target case's keyed `loads` delta.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on `case_id`, the same code on
//! `load_id`, `mutation.target-mismatch` (Error) when the replacement renames the load,
//! the SAME per-variant target resolution `add-load` runs (`mutation.target-missing`, Error), the
//! finite-magnitude bound (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceLoad;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dLoadCasePatch, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry, Fem2dLoadsDelta, Fem2dLoadsPatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::{load_id, Fem2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceLoad, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    let Some(held) = existing.loads.iter().find(|load| load_id(load) == payload.load_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load \"{}\" does not exist in case \"{}\".", payload.load_id, payload.case_id), [payload.load_id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("load", &payload.load_id, load_id(&payload.new_load)) {
        return rejection;
    }
    if let Some(rejection) = guards::load_reference(base, &payload.new_load) {
        return rejection;
    }
    if let Some(rejection) = guards::load_magnitudes(&payload.new_load) {
        return rejection;
    }
    if *held == *payload.new_load {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Load \"{}\" in case \"{}\" is already equal to the replacement value.", payload.load_id, payload.case_id));
    }
    let loads = Fem2dLoadsDelta { patched: vec![Fem2dLoadsPatchEntry { id: payload.load_id.clone(), item: payload.new_load.clone() }], ..Default::default() };
    protocol::MutationOutcome::new(Fem2dDiff {
        load_cases: Some(Fem2dLoadCasesDelta { patched: vec![Fem2dLoadCasesPatchEntry { id: payload.case_id.clone(), patch: Fem2dLoadCasePatch { loads: Some(loads), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
