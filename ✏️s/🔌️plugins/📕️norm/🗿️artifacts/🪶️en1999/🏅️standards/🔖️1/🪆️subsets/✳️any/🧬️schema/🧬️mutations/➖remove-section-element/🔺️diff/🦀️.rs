//! 🔺️ `remove-section-element` diff.

use crate::remove_section_element::RemoveSectionElement;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SectionsPatch, En1999SectionsRows, En1999SectionsElementsRows};

pub fn diff(payload: &RemoveSectionElement, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.sections.iter().find(|row| row.id == payload.section_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown section id {}", payload.section_id), vec![payload.section_id.clone()]);
    };
    let Some(index) = parent.elements.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown section element id {}", payload.id), Vec::<String>::new());
    };
    let nested = En1999SectionsElementsRows::removal(&parent.elements, index);
    protocol::MutationOutcome::new(En1999Diff { sections: Some(En1999SectionsRows::modification(payload.section_id.clone(), En1999SectionsPatch { elements: Some(nested), ..Default::default() })), ..Default::default() })
}
