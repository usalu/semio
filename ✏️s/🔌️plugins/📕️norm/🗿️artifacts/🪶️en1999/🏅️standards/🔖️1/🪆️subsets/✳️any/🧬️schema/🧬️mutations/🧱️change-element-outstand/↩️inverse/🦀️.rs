//! ↩️ `change-element-outstand` inverse: restores the previous `outstand`.

use crate::change_element_outstand::ChangeElementOutstand;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeElementOutstand, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_outstand::ChangeElementOutstand> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id).and_then(|row| row.elements.iter().find(|row| row.id == payload.element_id)) {
        Some(row) => vec![Op::from(ChangeElementOutstand { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_outstand: row.outstand })],
        None => Vec::new(),
    })
}
