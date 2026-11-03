//! ↩️ Inverse for `change-column-concrete-f-ck-pa`.
use crate::change_column_concrete_f_ck_pa::ChangeColumnConcreteFCkPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnConcreteFCkPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_concrete_f_ck_pa::ChangeColumnConcreteFCkPa> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnConcreteFCkPa { id: payload.id.clone(), new_concrete_f_ck_pa: column.concrete_f_ck_pa })])
}
