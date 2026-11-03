//! 🔺️ Diff for `change-column-action-moment-nm`.
use crate::change_column_action_moment_nm::ChangeColumnActionMomentNm;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994ColumnsRows, En1994ColumnsPatch, En1994ColumnsActionsRows, En1994ColumnsActionsPatch};

pub fn diff(payload: &ChangeColumnActionMomentNm, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.column_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("column {} does not exist.", payload.column_id), [payload.column_id.clone()]);
    };
    let Some(action_index) = base.columns[index].actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.new_m_k_nm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [index.to_string()]);
    }
    let Some(column) = base.columns.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "column missing", [index.to_string()]);
    };
    let Some(action) = column.actions.get(action_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action missing", [action_index.to_string()]);
    };
    if (action.m_k_nm - payload.new_m_k_nm).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        columns: Some(En1994ColumnsRows::modification(&payload.column_id, En1994ColumnsPatch {
                actions: Some(En1994ColumnsActionsRows::modification(&payload.id, En1994ColumnsActionsPatch { m_k_nm: Some(payload.new_m_k_nm), ..Default::default() })),
                ..Default::default()
            })),
        ..Default::default()
    })
}
