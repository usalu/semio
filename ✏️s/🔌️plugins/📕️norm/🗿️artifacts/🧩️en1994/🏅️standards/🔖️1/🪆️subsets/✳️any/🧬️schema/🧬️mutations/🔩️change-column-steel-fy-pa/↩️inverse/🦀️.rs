//! ↩️ Inverse for `change-column-steel-fy-pa`.
use crate::change_column_steel_f_y_pa::ChangeColumnSteelFYPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnSteelFYPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_steel_f_y_pa::ChangeColumnSteelFYPa> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnSteelFYPa { id: payload.id.clone(), new_steel_f_y_pa: column.steel_f_y_pa })])
}
