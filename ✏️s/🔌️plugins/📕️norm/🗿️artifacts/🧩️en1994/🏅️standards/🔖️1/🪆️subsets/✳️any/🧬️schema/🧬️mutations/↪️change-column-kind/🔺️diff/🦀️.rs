//! 🔺️ Diff for `change-column-kind`.
use crate::change_column_kind::ChangeColumnKind;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994ColumnsRows, En1994ColumnsPatch};

pub fn diff(payload: &ChangeColumnKind, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(col) = base.columns.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "column missing", [payload.index.to_string()]);
    };
    if col.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        columns: Some(En1994ColumnsRows { modified: vec![En1994ColumnsPatch { index: payload.index, kind: Some(payload.new_kind.clone()), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
