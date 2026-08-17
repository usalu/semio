//! 🔺️ Sparse diff builder for `AddLoad` — clones the target case, pushes the load, patches it.
use super::mutation::AddLoad;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry};
use crate::artifacts::fem2d::{load_id, Fem2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &AddLoad, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    let new_load_id = load_id(&payload.load);
    if existing.loads.iter().any(|load| load_id(load) == new_load_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Load \"{}\" already exists in case \"{}\".", new_load_id, payload.case_id));
    }
    let mut item = existing.clone();
    item.loads.push((*payload.load).clone());
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { patched: vec![Fem2dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
