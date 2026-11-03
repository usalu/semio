//! 🔺️ Diff for `change-column-concrete-area-m2`.
use crate::change_column_concrete_area_m2::ChangeColumnConcreteAreaM2;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994ColumnsRows, En1994ColumnsPatch};

pub fn diff(payload: &ChangeColumnConcreteAreaM2, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("column {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_concrete_a_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(column) = base.columns.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "column missing", [index.to_string()]);
    };
    if (column.concrete_a_m2 - payload.new_concrete_a_m2).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        columns: Some(En1994ColumnsRows::modification(&payload.id, En1994ColumnsPatch { concrete_a_m2: Some(payload.new_concrete_a_m2), ..Default::default() })),
        ..Default::default()
    })
}
