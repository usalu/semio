//! ↩️ Inverse for `change-column-rebar-cover-m`.
use crate::change_column_rebar_cover_m::ChangeColumnRebarCoverM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnRebarCoverM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_rebar_cover_m::ChangeColumnRebarCoverM> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnRebarCoverM { id: payload.id.clone(), new_rebar_cover_m: column.rebar_cover_m })])
}
