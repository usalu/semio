//! ↩️ Inverse for `change-column-rebar-f-yk-pa`.
use crate::change_column_rebar_f_yk_pa::ChangeColumnRebarFYkPa;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnRebarFYkPa, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_rebar_f_yk_pa::ChangeColumnRebarFYkPa> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnRebarFYkPa { id: payload.id.clone(), new_reinforcement_f_yk_pa: column.reinforcement_f_yk_pa })])
}
