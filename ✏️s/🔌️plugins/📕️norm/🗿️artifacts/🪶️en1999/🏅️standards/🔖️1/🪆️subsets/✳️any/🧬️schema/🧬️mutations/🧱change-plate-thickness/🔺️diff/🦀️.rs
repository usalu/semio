//! 🔺️ `change-plate-thickness` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_plate_thickness::ChangePlateThickness;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangePlateThickness, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let mut sections = base.sections.clone();
    let Some(sec) = sections.iter_mut().find(|s| s.id == payload.section_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown section {}", payload.section_id), Vec::<String>::new());
    };
    let Some(el) = sec.elements.iter_mut().find(|e| e.id == payload.element_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown element {}", payload.element_id), Vec::<String>::new());
    };
    el.thickness = payload.new_thickness;
    protocol::MutationOutcome::new(En1999Diff { sections: Some(sections), ..Default::default() })
}
