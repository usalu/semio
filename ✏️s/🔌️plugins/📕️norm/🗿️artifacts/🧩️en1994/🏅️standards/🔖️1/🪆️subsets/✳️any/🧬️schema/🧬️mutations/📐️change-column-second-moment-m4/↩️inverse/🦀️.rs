//! ↩️ Inverse for `change-column-second-moment-m4`.
use crate::change_column_second_moment_m4::ChangeColumnSecondMomentM4;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnSecondMomentM4, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_second_moment_m4::ChangeColumnSecondMomentM4> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnSecondMomentM4 { id: payload.id.clone(), new_i_m4: column.i_m4 })])
}
