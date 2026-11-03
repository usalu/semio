//! ↩️ `change-section-outer-diameter` inverse: restores the previous `outer_diameter`.

use crate::change_section_outer_diameter::ChangeSectionOuterDiameter;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeSectionOuterDiameter, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_section_outer_diameter::ChangeSectionOuterDiameter> {
    Ok(match base.sections.iter().find(|row| row.id == payload.section_id) {
        Some(row) => vec![Op::from(ChangeSectionOuterDiameter { section_id: payload.section_id.clone(), new_outer_diameter: row.outer_diameter })],
        None => Vec::new(),
    })
}
