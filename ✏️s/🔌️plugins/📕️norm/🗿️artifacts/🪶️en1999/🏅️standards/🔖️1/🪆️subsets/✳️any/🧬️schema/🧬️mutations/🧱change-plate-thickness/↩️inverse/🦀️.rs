//! ↩️ `change-plate-thickness` inverse.

use crate::change_plate_thickness::ChangePlateThickness;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePlateThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plate_thickness::ChangePlateThickness> {
    Ok((|| {
    let t = base.sections.iter().find(|s| s.id == payload.section_id)
        .and_then(|s| s.elements.iter().find(|e| e.id == payload.element_id)).map(|e| e.thickness).unwrap_or(0.0);
    vec![Op::from(ChangePlateThickness { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_thickness: t })]

    })())
}
