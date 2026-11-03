//! ↩️ Inverse for `change-column-outer-size-m`.
use crate::change_column_outer_size_m::ChangeColumnOuterSizeM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnOuterSizeM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_outer_size_m::ChangeColumnOuterSizeM> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnOuterSizeM { id: payload.id.clone(), new_outer_size_m: column.outer_size_m })])
}
