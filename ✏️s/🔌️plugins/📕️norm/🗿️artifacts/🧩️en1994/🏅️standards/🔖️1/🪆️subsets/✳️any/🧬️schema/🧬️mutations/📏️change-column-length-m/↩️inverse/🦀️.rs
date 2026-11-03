//! ↩️ Inverse for `change-column-length-m`.
use crate::change_column_length_m::ChangeColumnLengthM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnLengthM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_length_m::ChangeColumnLengthM> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnLengthM { id: payload.id.clone(), new_length_m: column.length_m })])
}
