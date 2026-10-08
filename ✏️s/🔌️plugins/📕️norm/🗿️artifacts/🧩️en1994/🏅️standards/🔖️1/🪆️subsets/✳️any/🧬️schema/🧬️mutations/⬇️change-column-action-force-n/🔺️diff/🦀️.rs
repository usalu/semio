//! Diff for `change-column-action-force-n`.
use super::ChangeColumnActionForceN;
use crate::{En1994Snapshot};
use crate::diff::{En1994Diff, En1994ColumnsRows, En1994ColumnsPatch, En1994ColumnsActionsRows, En1994ColumnsActionsPatch};

pub fn diff(payload: &ChangeColumnActionForceN, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_n_k_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(col) = base.columns.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "column missing", [payload.index.to_string()]);
    };
    let Some(action) = col.actions.get(payload.action_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action missing", [payload.action_index.to_string()]);
    };
    if (action.n_k_n - payload.new_n_k_n).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        columns: Some(En1994ColumnsRows {
            modified: vec![En1994ColumnsPatch {
                index: payload.index,
                actions: Some(En1994ColumnsActionsRows { modified: vec![En1994ColumnsActionsPatch { index: payload.action_index, n_k_n: Some(payload.new_n_k_n), ..Default::default() }] }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
