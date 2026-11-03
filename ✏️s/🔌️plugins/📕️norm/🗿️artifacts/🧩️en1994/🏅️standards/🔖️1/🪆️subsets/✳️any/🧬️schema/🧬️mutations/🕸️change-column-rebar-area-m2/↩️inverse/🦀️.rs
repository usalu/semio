//! ↩️ Inverse for `change-column-rebar-area-m2`.
use crate::change_column_rebar_area_m2::ChangeColumnRebarAreaM2;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnRebarAreaM2, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_rebar_area_m2::ChangeColumnRebarAreaM2> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnRebarAreaM2 { id: payload.id.clone(), new_reinforcement_as_m2: column.reinforcement_as_m2 })])
}
