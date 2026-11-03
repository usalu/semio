use crate::insert_parametric_section::InsertParametricSection;
use crate::remove_parametric_section;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertParametricSection, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_parametric_section::RemoveParametricSection> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.parametric_sections.len());
    vec![Op::from(remove_parametric_section::RemoveParametricSection { index: at })]

    })())
}
