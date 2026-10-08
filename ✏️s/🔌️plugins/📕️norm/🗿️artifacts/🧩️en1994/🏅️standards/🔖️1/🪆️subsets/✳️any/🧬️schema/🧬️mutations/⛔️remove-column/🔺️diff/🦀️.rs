//! Diff for `remove-column`.
use super::RemoveColumn;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994ColumnsRows};

pub fn diff(payload: &RemoveColumn, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index >= base.columns.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnsRows { removed: vec![payload.index], ..Default::default() }), ..Default::default() })
}
