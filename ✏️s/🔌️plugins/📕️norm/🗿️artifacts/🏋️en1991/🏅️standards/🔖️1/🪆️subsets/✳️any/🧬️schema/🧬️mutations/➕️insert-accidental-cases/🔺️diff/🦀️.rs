//! 🔺️ Diff for `insert-accidental-cases`.
use super::InsertAccidentalCases;
use crate::{En1991Diff, En1991Snapshot};
use crate::diff::{En1991AccidentalCaseDelta};
pub fn diff(payload: &InsertAccidentalCases, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.accidental_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.accidental_cases.iter().any(|existing| existing.id == payload.item.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Row id {} already exists.", payload.item.id), [payload.item.id.clone()]);
    }
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: En1991AccidentalCaseDelta::insertion(&base.accidental_cases, payload.index, payload.item.clone()), ..Default::default() })
}
