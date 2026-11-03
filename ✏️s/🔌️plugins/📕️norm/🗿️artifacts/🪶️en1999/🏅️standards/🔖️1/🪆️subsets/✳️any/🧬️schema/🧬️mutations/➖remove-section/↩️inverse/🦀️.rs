//! ↩️ `remove-section` inverse.

use crate::remove_section::RemoveSection;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_section;

pub fn inverse<Op>(payload: &RemoveSection, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_section::InsertSection> {
    Ok((|| {
    if let Some((index, row)) = base.sections.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_section::InsertSection { index: index as u32, section: row.clone() })]
    } else { Vec::new() }

    })())
}
