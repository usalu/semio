//! Diff for `insert-accidental-cases`.
use super::InsertAccidentalCases;
use crate::artifact_schema::diff::En1991AccidentalCasesList;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &InsertAccidentalCases, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index > base.accidental_cases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Index out of range.", Vec::<String>::new());
    }
    let mut values = base.accidental_cases.clone();
    values.insert(payload.index, payload.item.clone());
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: Some(En1991AccidentalCasesList { values }), ..Default::default() })
}
