//! 🔺️ Sparse diff builder for `AddLoad` — clones the target case, pushes the load, patches it.
use super::AddLoad;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dLoadCasePatch, Fem3dLoadCasesDelta, Fem3dLoadCasesModification, Fem3dLoadInsertion, Fem3dLoadsDelta};
use crate::standards::v1::subsets::any::schema::mutations::resolve_load;
use crate::{load_id, Fem3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &AddLoad, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    let new_load_id = load_id(&payload.load);
    if existing.loads.iter().any(|load| load_id(load) == new_load_id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Load \"{}\" already exists in case \"{}\".", new_load_id, payload.case_id));
    }
    if let Some(refusal) = resolve_load(base, &payload.load) {
        return refusal;
    }
    if payload.index.is_some_and(|at| at > existing.loads.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), existing.loads.len()), [new_load_id.to_string()]);
    }
    let loads = Fem3dLoadsDelta { inserted: vec![Fem3dLoadInsertion { index: payload.index.unwrap_or(existing.loads.len()), row: (*payload.load).clone() }], ..Default::default() };
    protocol::MutationOutcome::new(Fem3dDiff {
        load_cases: Some(Fem3dLoadCasesDelta { modified: vec![Fem3dLoadCasesModification { id: payload.case_id.clone(), patch: Fem3dLoadCasePatch { loads: Some(loads), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
