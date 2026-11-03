//! ↩️ Inverse for `change-column-action-moment-nm`.
use crate::change_column_action_moment_nm::ChangeColumnActionMomentNm;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnActionMomentNm, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_action_moment_nm::ChangeColumnActionMomentNm> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.column_id) else {
        return Ok(Vec::new());
    };
    let Some(action_index) = base.columns[index].actions.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    let Some(action) = column.actions.get(action_index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnActionMomentNm { column_id: payload.column_id.clone(), id: payload.id.clone(), new_m_k_nm: action.m_k_nm })])
}
