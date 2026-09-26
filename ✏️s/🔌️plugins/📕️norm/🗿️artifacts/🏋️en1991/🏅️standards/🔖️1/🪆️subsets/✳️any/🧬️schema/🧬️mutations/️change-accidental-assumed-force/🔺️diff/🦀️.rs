//! Diff for `change-accidental-assumed-force`.
use super::ChangeAccidentalAssumedForce;
use crate::artifact_schema::diff::En1991AccidentalCasesList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAccidentalAssumedForce, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.accidental_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    if base.accidental_cases[payload.index].impact.first().map(|i| i.assumed_force) == Some(payload.new_assumed_force) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    let mut values = base.accidental_cases.clone();
    if let Some(imp) = values[payload.index].impact.first_mut() { imp.assumed_force = payload.new_assumed_force; } else { return protocol::MutationOutcome::fatal("mutation.invariant", "Impact variant missing.", Vec::<String>::new()); }
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: Some(En1991AccidentalCasesList { values }), ..Default::default() })
}
