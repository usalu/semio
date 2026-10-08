//! Diff for `insert-column`.
use super::InsertColumn;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994ColumnsRows, En1994ColumnsInserted};

pub fn diff(payload: &InsertColumn, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index > base.columns.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnsRows { inserted: vec![En1994ColumnsInserted { index: payload.index, row: payload.column.clone() }], ..Default::default() }), ..Default::default() })
}
