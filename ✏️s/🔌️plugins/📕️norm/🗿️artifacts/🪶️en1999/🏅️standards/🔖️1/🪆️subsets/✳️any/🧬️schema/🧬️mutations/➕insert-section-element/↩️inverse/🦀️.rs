//! ↩️ `insert-section-element` inverse.

use crate::insert_section_element::InsertSectionElement;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_section_element;

pub fn inverse<Op>(payload: &InsertSectionElement, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_section_element::RemoveSectionElement> {
    Ok((|| {
    vec![Op::from(remove_section_element::RemoveSectionElement { section_id: payload.section_id.clone(), id: payload.element.id.clone() })]
    })())
}
