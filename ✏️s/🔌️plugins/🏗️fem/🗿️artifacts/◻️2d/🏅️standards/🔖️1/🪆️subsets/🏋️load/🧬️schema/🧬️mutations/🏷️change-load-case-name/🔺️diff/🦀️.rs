//! 🔺️ Sparse diff builder for `ChangeLoadCaseName`.
use super::ChangeLoadCaseName;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dLoadCasesDelta, Fem2dLoadCasesPatchEntry};
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLoadCaseName, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.load_cases.iter().find(|case| case.id == payload.case_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.case_id), [payload.case_id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Load case \"{}\" is already named \"{}\".", payload.case_id, payload.new_name));
    }
    let mut item = existing.clone();
    item.name.clone_from(&payload.new_name);
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { patched: vec![Fem2dLoadCasesPatchEntry { id: payload.case_id.clone(), item }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
