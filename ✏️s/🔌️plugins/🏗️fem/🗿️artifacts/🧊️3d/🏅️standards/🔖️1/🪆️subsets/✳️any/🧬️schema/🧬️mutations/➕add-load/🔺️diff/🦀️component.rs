//! 🔺️ Sparse diff builder for `AddLoad` — clones the target case, pushes the load, patches it.
use super::mutation::AddLoad;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &AddLoad, base: &Fem3dSnapshot) -> Fem3dDiff {
    let mut diff = Fem3dDiff::default();
    if let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) {
        let mut item = existing.clone();
        item.loads.push((*payload.load).clone());
        diff.load_cases = Some(Fem3dLoadCasesDelta { patched: vec![Fem3dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() });
    }
    diff
}
//#endregion 🔖️Diff
