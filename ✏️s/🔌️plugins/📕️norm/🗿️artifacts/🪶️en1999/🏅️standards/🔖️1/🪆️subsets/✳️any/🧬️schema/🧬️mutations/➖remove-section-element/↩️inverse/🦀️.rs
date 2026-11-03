//! ↩️ `remove-section-element` inverse.

use crate::remove_section_element::RemoveSectionElement;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_section_element;

pub fn inverse<Op>(payload: &RemoveSectionElement, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_section_element::InsertSectionElement> {
    Ok((|| {
    match base.sections.iter().find(|row| row.id == payload.section_id).and_then(|parent| parent.elements.iter().enumerate().find(|(_, row)| row.id == payload.id)) {
        Some((index, row)) => vec![Op::from(insert_section_element::InsertSectionElement { section_id: payload.section_id.clone(), index: index as u32, element: row.clone() })],
        None => Vec::new(),
    }
    })())
}
