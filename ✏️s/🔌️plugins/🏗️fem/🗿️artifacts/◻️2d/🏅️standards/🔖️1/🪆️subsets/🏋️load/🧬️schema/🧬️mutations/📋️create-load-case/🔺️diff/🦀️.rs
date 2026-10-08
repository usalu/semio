//! 🔺️ Sparse diff builder for `CreateLoadCase`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::load_reference` resolution of every carried load's own target, in payload order
//! (`mutation.target-missing`, Error). `add-load` calls the SAME guard, so the two doors into a
//! case's `loads` cannot drift apart.
use super::CreateLoadCase;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dLoadCaseInsertion, Fem2dLoadCasesDelta};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateLoadCase, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.load_cases.iter().any(|case| case.id == payload.load_case.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A load case with id \"{}\" already exists.", payload.load_case.id), [payload.load_case.id.clone()]);
    }
    for load in &payload.load_case.loads {
        if let Some(rejection) = guards::load_reference(base, load) {
            return rejection;
        }
    }
    if payload.index.is_some_and(|at| at > base.load_cases.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.load_cases.len()), [&payload.load_case.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { inserted: vec![Fem2dLoadCaseInsertion { index: payload.index.unwrap_or(base.load_cases.len()), row: payload.load_case.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
