//! 🔺️ Sparse diff builder for `RemoveLoad` — clones the target case, drops the load, patches it.
use super::mutation::RemoveLoad;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry};
use crate::artifacts::fem2d::{load_id, Fem2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveLoad, base: &Fem2dSnapshot) -> Fem2dDiff {
    let mut diff = Fem2dDiff::default();
    if let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) {
        let mut item = existing.clone();
        item.loads.retain(|load| load_id(load) != payload.load_id);
        diff.load_cases = Some(Fem2dLoadCasesDelta { patched: vec![Fem2dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() });
    }
    diff
}
//#endregion 🔖️Diff
