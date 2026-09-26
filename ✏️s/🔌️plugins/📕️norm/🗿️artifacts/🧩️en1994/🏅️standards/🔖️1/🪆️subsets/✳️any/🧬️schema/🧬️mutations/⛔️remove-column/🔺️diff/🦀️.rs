//! Diff for `remove-column`.
use super::RemoveColumn;
use crate::diff::En1994ColumnList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &RemoveColumn, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index >= base.columns.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    let mut columns = base.columns.clone();
    columns.remove(payload.index);
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnList { values: columns }), ..Default::default() })
}
