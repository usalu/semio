//! 🔺️ Sparse diff builder for `CreateLoadCase`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::load_reference` resolution of every carried load's own target, in payload order
//! (`mutation.target-missing`, Error). `add-load` calls the SAME guard, so the two doors into a
//! case's `loads` cannot drift apart.
use super::CreateLoadCase;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dLoadCasesDelta};
use crate::artifacts::fem2d::mutations::guards;
use crate::artifacts::fem2d::Fem2dSnapshot;

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
    protocol::MutationOutcome::new(Fem2dDiff { load_cases: Some(Fem2dLoadCasesDelta { added: vec![payload.load_case.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
