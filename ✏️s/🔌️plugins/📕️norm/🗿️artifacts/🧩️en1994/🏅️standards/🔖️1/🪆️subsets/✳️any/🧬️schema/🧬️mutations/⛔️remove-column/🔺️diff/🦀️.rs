//! 🔺️ Diff for `remove-column`.
use crate::remove_column::RemoveColumn;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994ColumnsRows};

pub fn diff(payload: &RemoveColumn, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("column {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if index >= base.columns.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnsRows::removal(&base.columns, index)), ..Default::default() })
}
