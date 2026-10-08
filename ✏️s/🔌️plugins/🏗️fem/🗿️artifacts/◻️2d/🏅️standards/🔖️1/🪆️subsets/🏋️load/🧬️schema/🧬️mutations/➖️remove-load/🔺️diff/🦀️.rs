//! 🔺️ Sparse diff builder for `RemoveLoad` — clones the target case, drops the load, patches it.
use super::RemoveLoad;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dLoadCasePatch, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry, Fem2dLoadsDelta};
use crate::{load_id, Fem2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveLoad, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    if !existing.loads.iter().any(|load| load_id(load) == payload.load_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load \"{}\" does not exist in case \"{}\".", payload.load_id, payload.case_id), [payload.load_id.clone()]);
    }
    let loads = Fem2dLoadsDelta { removed: vec![payload.load_id.clone()], ..Default::default() };
    protocol::MutationOutcome::new(Fem2dDiff {
        load_cases: Some(Fem2dLoadCasesDelta { patched: vec![Fem2dLoadCasesPatchEntry { id: payload.case_id.clone(), patch: Fem2dLoadCasePatch { loads: Some(loads), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
