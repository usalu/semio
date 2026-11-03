//! ↩️ `insert-section` inverse.

use crate::insert_section::InsertSection;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_section;

pub fn inverse<Op>(payload: &InsertSection, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_section::RemoveSection> {
    Ok((|| {
    vec![Op::from(remove_section::RemoveSection { id: payload.section.id.clone() })]

    })())
}
