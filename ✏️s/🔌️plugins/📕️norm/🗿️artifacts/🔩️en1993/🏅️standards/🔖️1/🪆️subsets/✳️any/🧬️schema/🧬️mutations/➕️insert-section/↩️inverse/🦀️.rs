use crate::insert_section::InsertSection;
use crate::remove_section;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertSection, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_section::RemoveSection> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.sections.len());
    vec![Op::from(remove_section::RemoveSection { index: at })]

    })())
}
