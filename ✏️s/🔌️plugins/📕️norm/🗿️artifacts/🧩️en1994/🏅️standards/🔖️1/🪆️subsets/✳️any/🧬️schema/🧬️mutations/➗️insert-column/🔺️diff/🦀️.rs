//! 🔺️ Diff for `insert-column`.
use crate::insert_column::InsertColumn;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994ColumnsRows, En1994ColumnsInserted};

pub fn diff(payload: &InsertColumn, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if payload.index > base.columns.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [payload.index.to_string()]);
    }
    if base.columns.iter().any(|row| row.id == payload.column.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A column with id '{}' already exists.", payload.column.id), [payload.column.id.clone()]);
    }
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnsRows::insertion(payload.index, payload.column.clone())), ..Default::default() })
}
