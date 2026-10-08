//! 🔺️ `change-plate-thickness` diff.

use crate::mutations::change_plate_thickness::ChangePlateThickness;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999SectionsRows, En1999SectionsPatch, En1999SectionsElementsRows, En1999SectionsElementsPatch};

pub fn diff(payload: &ChangePlateThickness, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(section) = base.sections.iter().find(|s| s.id == payload.section_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown section {}", payload.section_id), Vec::<String>::new());
    };
    if !section.elements.iter().any(|e| e.id == payload.element_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown element {}", payload.element_id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff {
        sections: Some(En1999SectionsRows {
            modified: vec![En1999SectionsPatch {
                id: payload.section_id.clone(),
                elements: Some(En1999SectionsElementsRows { modified: vec![En1999SectionsElementsPatch { id: payload.element_id.clone(), thickness: Some(payload.new_thickness), ..Default::default() }] }),
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
