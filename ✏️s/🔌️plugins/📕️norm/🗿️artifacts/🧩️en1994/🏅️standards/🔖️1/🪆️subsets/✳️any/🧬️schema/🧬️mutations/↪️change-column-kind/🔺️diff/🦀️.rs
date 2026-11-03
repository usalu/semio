//! 🔺️ Diff for `change-column-kind`.
use crate::change_column_kind::ChangeColumnKind;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994ColumnsRows, En1994ColumnsPatch};

pub fn diff(payload: &ChangeColumnKind, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("column {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(col) = base.columns.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "column missing", [index.to_string()]);
    };
    if col.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        columns: Some(En1994ColumnsRows::modification(&payload.id, En1994ColumnsPatch { kind: Some(payload.new_kind), ..Default::default() })),
        ..Default::default()
    })
}
