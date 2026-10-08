//! 🔺️ Diff for `remove-accidental-cases`.
use super::RemoveAccidentalCases;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991AccidentalCaseDelta};
pub fn diff(payload: &RemoveAccidentalCases, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.accidental_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: En1991AccidentalCaseDelta::removal(&base.accidental_cases, payload.index), ..Default::default() })
}
