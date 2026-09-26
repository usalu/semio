//! ↩️ `change-plate-thickness` inverse.

use crate::mutations::change_plate_thickness::ChangePlateThickness;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(payload: &ChangePlateThickness, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    let t = base.sections.iter().find(|s| s.id == payload.section_id)
        .and_then(|s| s.elements.iter().find(|e| e.id == payload.element_id)).map(|e| e.thickness).unwrap_or(0.0);
    vec![En1999Mutation::ChangePlateThickness(ChangePlateThickness { section_id: payload.section_id.clone(), element_id: payload.element_id.clone(), new_thickness: t })]
}
