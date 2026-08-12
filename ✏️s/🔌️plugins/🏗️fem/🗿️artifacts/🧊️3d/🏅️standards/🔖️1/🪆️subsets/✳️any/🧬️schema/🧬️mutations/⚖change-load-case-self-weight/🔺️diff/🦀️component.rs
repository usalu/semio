//! 🔺️ Sparse diff builder for `ChangeLoadCaseSelfWeight`.
use super::mutation::ChangeLoadCaseSelfWeight;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadCaseSelfWeight, base: &Fem3dSnapshot) -> Fem3dDiff {
    let mut diff = Fem3dDiff::default();
    if let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) {
        let mut item = existing.clone();
        item.self_weight = payload.new_self_weight;
        diff.load_cases = Some(Fem3dLoadCasesDelta { patched: vec![Fem3dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() });
    }
    diff
}
//#endregion 🔖️Diff
