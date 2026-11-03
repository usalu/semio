//! ↩️ Inverse for `change-column-concrete-area-m2`.
use crate::change_column_concrete_area_m2::ChangeColumnConcreteAreaM2;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnConcreteAreaM2, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_concrete_area_m2::ChangeColumnConcreteAreaM2> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnConcreteAreaM2 { id: payload.id.clone(), new_concrete_a_m2: column.concrete_a_m2 })])
}
