//! 🔺️ Diff for `change-column-introduced-force-n`.
use crate::change_column_introduced_force_n::ChangeColumnIntroducedForceN;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994ColumnsRows, En1994ColumnsPatch};

pub fn diff(payload: &ChangeColumnIntroducedForceN, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("column {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_introduced_force_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(column) = base.columns.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "column missing", [index.to_string()]);
    };
    if (column.introduced_force_n - payload.new_introduced_force_n).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        columns: Some(En1994ColumnsRows::modification(&payload.id, En1994ColumnsPatch { introduced_force_n: Some(payload.new_introduced_force_n), ..Default::default() })),
        ..Default::default()
    })
}
