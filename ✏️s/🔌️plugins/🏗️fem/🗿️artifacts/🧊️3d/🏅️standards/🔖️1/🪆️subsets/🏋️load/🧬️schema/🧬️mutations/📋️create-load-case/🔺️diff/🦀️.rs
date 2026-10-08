//! 🔺️ Sparse diff builder for `CreateLoadCase`.
use super::CreateLoadCase;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dLoadCaseInsertion, Fem3dLoadCasesDelta};
use crate::standards::v1::subsets::any::schema::mutations::resolve_load;
use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateLoadCase, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if base.load_cases.iter().any(|case| case.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A load case with id \"{}\" already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    for load in &payload.load_case.loads {
        if let Some(refusal) = resolve_load(base, load) {
            return refusal;
        }
    }
    if payload.index.is_some_and(|at| at > base.load_cases.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.load_cases.len()), [&payload.load_case.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { load_cases: Some(Fem3dLoadCasesDelta { inserted: vec![Fem3dLoadCaseInsertion { index: payload.index.unwrap_or(base.load_cases.len()), row: payload.load_case.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
