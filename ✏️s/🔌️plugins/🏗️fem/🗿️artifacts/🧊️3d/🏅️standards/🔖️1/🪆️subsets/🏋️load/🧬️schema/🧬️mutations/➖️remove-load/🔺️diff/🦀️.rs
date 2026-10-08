//! 🔺️ Sparse diff builder for `RemoveLoad` — clones the target case, drops the load, patches it.
use super::RemoveLoad;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dLoadCasePatch, Fem3dLoadCasesDelta, Fem3dLoadCasesModification, Fem3dLoadsDelta, Fem3dLoadRemoval};
use crate::{load_id, Fem3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveLoad, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    let Some(at) = existing.loads.iter().position(|load| load_id(load) == payload.load_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load \"{}\" does not exist in case \"{}\".", payload.load_id, payload.case_id), [payload.load_id.clone()]);
    };
    let loads = Fem3dLoadsDelta { removed: vec![Fem3dLoadRemoval { id: payload.load_id.clone(), index: at }], ..Default::default() };
    protocol::MutationOutcome::new(Fem3dDiff {
        load_cases: Some(Fem3dLoadCasesDelta { modified: vec![Fem3dLoadCasesModification { id: payload.case_id.clone(), patch: Fem3dLoadCasePatch { loads: Some(loads), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
