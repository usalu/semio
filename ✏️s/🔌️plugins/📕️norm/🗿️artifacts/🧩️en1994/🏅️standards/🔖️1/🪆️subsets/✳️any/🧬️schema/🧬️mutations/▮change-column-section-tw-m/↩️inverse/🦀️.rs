//! ↩️ Inverse for `change-column-section-tw-m`.
use crate::change_column_section_tw_m::ChangeColumnSectionTwM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnSectionTwM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_section_tw_m::ChangeColumnSectionTwM> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnSectionTwM { id: payload.id.clone(), new_section_tw_m: column.section_tw_m })])
}
