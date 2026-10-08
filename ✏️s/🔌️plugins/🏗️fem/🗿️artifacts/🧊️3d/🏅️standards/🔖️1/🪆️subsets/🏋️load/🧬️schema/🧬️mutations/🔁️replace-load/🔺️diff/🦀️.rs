//! 🔺️ Sparse diff builder for `ReplaceLoad` — one patched load row inside the target case's keyed `loads` delta.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on `case_id`, the same code on
//! `load_id`, `mutation.target-mismatch` (Error) when the replacement renames the load, the SAME
//! per-variant target resolution `add-load` runs (`mutation.target-missing`, Error), the finite
//! magnitude bound (`mutation.invariant`, Fatal), and finally `mutation.no-op`.
use super::ReplaceLoad;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dLoadCasePatch, Fem3dLoadCasesDelta, Fem3dLoadCasesModification, Fem3dLoadsDelta, Fem3dLoadsModification};
use crate::standards::v1::subsets::any::schema::mutations::{target_mismatch,invariant,load_breach,resolve_load};

use crate::{load_id, Fem3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplaceLoad, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    let Some(held) = existing.loads.iter().find(|load| load_id(load) == payload.load_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load \"{}\" does not exist in case \"{}\".", payload.load_id, payload.case_id), [payload.load_id.clone()]);
    };
    if load_id(&payload.new_load) != payload.load_id {
        return target_mismatch("Load", &payload.load_id, load_id(&payload.new_load));
    }
    if let Some(refusal) = resolve_load(base, &payload.new_load) {
        return refusal;
    }
    if let Some(breach) = load_breach(&payload.new_load) {
        return invariant(breach, vec![payload.load_id.clone()]);
    }
    if *held == *payload.new_load {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Load \"{}\" in case \"{}\" already has that value.", payload.load_id, payload.case_id));
    }
    let loads = Fem3dLoadsDelta { modified: vec![Fem3dLoadsModification { id: payload.load_id.clone(), patch: payload.new_load.clone() }], ..Default::default() };
    protocol::MutationOutcome::new(Fem3dDiff {
        load_cases: Some(Fem3dLoadCasesDelta { modified: vec![Fem3dLoadCasesModification { id: payload.case_id.clone(), patch: Fem3dLoadCasePatch { loads: Some(loads), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
