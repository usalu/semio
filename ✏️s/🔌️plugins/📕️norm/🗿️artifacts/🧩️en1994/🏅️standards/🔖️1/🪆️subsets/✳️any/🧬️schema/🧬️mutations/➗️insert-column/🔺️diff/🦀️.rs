//! Diff for `insert-column`.
use super::InsertColumn;
use crate::diff::En1994ColumnList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &InsertColumn, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index > base.columns.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    let mut columns = base.columns.clone();
    columns.insert(payload.index, payload.column.clone());
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnList { values: columns }), ..Default::default() })
}
