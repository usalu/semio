//! ↩️ `change-section-flange-thickness` inverse: restores the previous `flange_thickness`.

use crate::change_section_flange_thickness::ChangeSectionFlangeThickness;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionFlangeThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_flange_thickness::ChangeSectionFlangeThickness> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id) {
        Some(row) => vec![Op::from(ChangeSectionFlangeThickness { section_id: payload.section_id.clone(), new_flange_thickness: row.flange_thickness })],
        None => Vec::new(),
    })
}
