//! ↩️ Inverse for `change-column-action-force-n`.
use crate::change_column_action_force_n::ChangeColumnActionForceN;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnActionForceN, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_action_force_n::ChangeColumnActionForceN> {
    Ok((|| {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.column_id) else {
        return Vec::new();
    };
    let Some(action_index) = base.columns[index].actions.iter().position(|row| row.id == payload.id) else {
        return Vec::new();
    };
    let Some(col) = base.columns.get(index) else { return Vec::new(); };
    let Some(action) = col.actions.get(action_index) else { return Vec::new(); };
    vec![Op::from(ChangeColumnActionForceN { column_id: payload.column_id.clone(), id: payload.id.clone(), new_n_k_n: action.n_k_n })]

    })())
}
