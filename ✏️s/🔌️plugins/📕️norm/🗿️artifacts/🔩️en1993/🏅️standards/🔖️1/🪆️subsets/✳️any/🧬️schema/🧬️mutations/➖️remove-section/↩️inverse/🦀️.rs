use crate::remove_section::RemoveSection;
use crate::insert_section;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveSection, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_section::InsertSection> {
    Ok((|| {
    if payload.index >= base.sections.len() { return Vec::new(); }
    vec![Op::from(insert_section::InsertSection { index: Some(payload.index), section: base.sections[payload.index].clone() })]

    })())
}
