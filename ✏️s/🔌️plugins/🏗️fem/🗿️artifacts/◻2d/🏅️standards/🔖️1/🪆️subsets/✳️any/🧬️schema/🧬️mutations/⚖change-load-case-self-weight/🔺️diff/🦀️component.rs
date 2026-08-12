//! 🔺️ Sparse diff builder for `ChangeLoadCaseSelfWeight`.
use super::mutation::ChangeLoadCaseSelfWeight;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadCaseSelfWeight, base: &Fem2dSnapshot) -> Fem2dDiff {
    let mut diff = Fem2dDiff::default();
    if let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) {
        let mut item = existing.clone();
        item.self_weight = payload.new_self_weight;
        diff.load_cases = Some(Fem2dLoadCasesDelta { patched: vec![Fem2dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() });
    }
    diff
}
//#endregion 🔖️Diff
