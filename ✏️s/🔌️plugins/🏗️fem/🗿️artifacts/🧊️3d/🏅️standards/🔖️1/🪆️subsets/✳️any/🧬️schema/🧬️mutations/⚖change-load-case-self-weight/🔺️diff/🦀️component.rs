//! 🔺️ Sparse diff builder for `ChangeLoadCaseSelfWeight`.
use super::mutation::ChangeLoadCaseSelfWeight;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeLoadCaseSelfWeight, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    if existing.self_weight == payload.new_self_weight {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Load case \"{}\" already has self-weight {}.", payload.case_id, payload.new_self_weight));
    }
    let mut item = existing.clone();
    item.self_weight = payload.new_self_weight;
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { patched: vec![Fem3dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
