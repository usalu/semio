//! 🔺️ Sparse diff builder for `DeleteLoadCase`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error), then
//! `mutation.target-referenced` (Error) while any combination still weights this case.
use super::DeleteLoadCase;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dLoadCasesDelta};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLoadCase, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.load_cases.iter().any(|case| case.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Load case \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some(rejection) = guards::referenced("Load case", "combination term", &payload.id, guards::load_case_referrers(base, &payload.id)) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
