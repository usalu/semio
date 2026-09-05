//! 🔺️ Sparse diff builder for `AddLoad` — clones the target case, pushes the load, patches it.
use super::AddLoad;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
use crate::artifacts::fem3d::{load_id, Fem3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &AddLoad, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    let new_load_id = load_id(&payload.load);
    if existing.loads.iter().any(|load| load_id(load) == new_load_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Load \"{}\" already exists in case \"{}\".", new_load_id, payload.case_id));
    }
    let mut item = existing.clone();
    item.loads.push((*payload.load).clone());
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { patched: vec![Fem3dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
