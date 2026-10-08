//! 🔺️ Diff for `change-accidental-assumed-force`.
use super::ChangeAccidentalAssumedForce;
use crate::diff::{En1991AccidentalCaseDelta, En1991AccidentalCasePatch};
use crate::{AccidentalImpact, En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAccidentalAssumedForce, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.accidental_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.accidental_cases[payload.index].impact.first().map(|i| i.assumed_force) == Some(payload.new_assumed_force) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    let case = &base.accidental_cases[payload.index];
    if case.impact.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Impact variant missing.", [payload.index.to_string()]);
    }
    let impact = case.impact.iter().enumerate().map(|(position, current)| if position == 0 { AccidentalImpact { assumed_force: payload.new_assumed_force, ..current.clone() } } else { current.clone() }).collect();
    protocol::MutationOutcome::new(En1991Diff {
        accidental_cases: En1991AccidentalCaseDelta::modification(&case.id, En1991AccidentalCasePatch { impact: Some(impact), ..Default::default() }),
        ..Default::default()
    })
}
