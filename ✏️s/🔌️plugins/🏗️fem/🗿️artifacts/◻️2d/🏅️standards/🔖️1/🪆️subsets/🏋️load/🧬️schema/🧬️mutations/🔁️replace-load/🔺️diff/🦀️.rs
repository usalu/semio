//! 🔺️ Sparse diff builder for `ReplaceLoad` — clones the target case, swaps the load in place,
//! patches it.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on `case_id`, the same code on
//! `load_id`, `mutation.id-mismatch` (Fatal) when the replacement renames the load,
//! the SAME per-variant target resolution `add-load` runs (`mutation.target-missing`, Error), the
//! finite-magnitude bound (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceLoad;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry};
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
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Load \"{}\" in case \"{}\" is already equal to the replacement value.", payload.load_id, payload.case_id));
    }
    let mut item = existing.clone();
    for slot in &mut item.loads {
        if load_id(slot) == payload.load_id {
            *slot = (*payload.new_load).clone();
        }
    }
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { patched: vec![Fem2dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
