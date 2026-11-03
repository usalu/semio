use crate::remove_parametric_section::RemoveParametricSection;
use crate::insert_parametric_section;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveParametricSection, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_parametric_section::InsertParametricSection> {
    Ok((|| {
    if payload.index >= base.parametric_sections.len() { return Vec::new(); }
    vec![Op::from(insert_parametric_section::InsertParametricSection { index: Some(payload.index), parametric_section: base.parametric_sections[payload.index].clone() })]

    })())
}
