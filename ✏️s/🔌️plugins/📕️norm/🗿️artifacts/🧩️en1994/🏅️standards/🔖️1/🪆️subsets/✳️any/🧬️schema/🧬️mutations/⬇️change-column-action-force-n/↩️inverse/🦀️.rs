//! Inverse for `change-column-action-force-n`.
use crate::change_column_action_force_n::ChangeColumnActionForceN;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnActionForceN, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_action_force_n::ChangeColumnActionForceN> {
    Ok((|| {
    let Some(col) = base.columns.get(payload.index) else { return Vec::new(); };
    let Some(action) = col.actions.get(payload.action_index) else { return Vec::new(); };
    vec![Op::from(ChangeColumnActionForceN { index: payload.index, action_index: payload.action_index, new_n_k_n: action.n_k_n })]

    })())
}
