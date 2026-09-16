//! 🔺️ Sparse diff builder for `ChangeLoadCaseName`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on `case_id`, then
//! `mutation.no-op` (Warning) for an unchanged name — no `Fatal` branch at all.
use super::ChangeLoadCaseName;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dLoadCasesDelta, Fem3dLoadCasesPatchEntry};
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadCaseName, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Load case \"{}\" is already named \"{}\".", payload.case_id, payload.new_name));
    }
    let mut item = existing.clone();
    item.name.clone_from(&payload.new_name);
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { patched: vec![Fem3dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
