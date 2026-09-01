//! 🔺️ Sparse diff builder for `ChangeLoadCaseSelfWeight`.
use super::ChangeLoadCaseSelfWeight;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadCaseSelfWeight, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    if existing.self_weight == payload.new_self_weight {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Load case \"{}\" self-weight is already {}.", payload.case_id, payload.new_self_weight));
    }
    let mut item = existing.clone();
    item.self_weight = payload.new_self_weight;
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { patched: vec![Fem2dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
