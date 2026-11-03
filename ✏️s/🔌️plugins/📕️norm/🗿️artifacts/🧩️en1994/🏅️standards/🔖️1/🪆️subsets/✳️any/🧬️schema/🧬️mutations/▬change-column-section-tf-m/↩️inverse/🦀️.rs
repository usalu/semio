//! ↩️ Inverse for `change-column-section-tf-m`.
use crate::change_column_section_tf_m::ChangeColumnSectionTfM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnSectionTfM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_section_tf_m::ChangeColumnSectionTfM> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnSectionTfM { id: payload.id.clone(), new_section_tf_m: column.section_tf_m })])
}
