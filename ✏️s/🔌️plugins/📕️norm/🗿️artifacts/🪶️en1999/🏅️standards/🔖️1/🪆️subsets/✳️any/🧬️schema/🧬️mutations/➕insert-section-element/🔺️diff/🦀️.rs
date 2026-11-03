//! 🔺️ `insert-section-element` diff.

use crate::insert_section_element::InsertSectionElement;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SectionsPatch, En1999SectionsRows, En1999SectionsElementsRows};

pub fn diff(payload: &InsertSectionElement, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.sections.iter().find(|row| row.id == payload.section_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown section id {}", payload.section_id), vec![payload.section_id.clone()]);
    };
    if parent.elements.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Section element id {} already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    if (payload.index as usize) > parent.elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.elements.len()), Vec::<String>::new());
    }
    let nested = En1999SectionsElementsRows::insertion(payload.index as usize, payload.element.clone());
    protocol::MutationOutcome::new(En1999Diff { sections: Some(En1999SectionsRows::modification(payload.section_id.clone(), En1999SectionsPatch { elements: Some(nested), ..Default::default() })), ..Default::default() })
}
