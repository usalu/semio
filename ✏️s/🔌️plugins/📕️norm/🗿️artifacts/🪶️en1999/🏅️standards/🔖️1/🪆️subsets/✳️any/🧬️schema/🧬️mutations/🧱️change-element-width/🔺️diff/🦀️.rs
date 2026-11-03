//! 🔺️ `change-element-width` diff: patches `width` of the addressed section element row.

use crate::change_element_width::ChangeElementWidth;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SectionsElementsPatch, En1999SectionsElementsRows, En1999SectionsPatch, En1999SectionsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeElementWidth, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_width.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.sections.iter().find(|row| row.id == payload.section_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "section", Vec::<String>::new());
    };
    let Some(r1) = r0.elements.iter().find(|row| row.id == payload.element_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "element", Vec::<String>::new());
    };
    if r1.width == payload.new_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section element already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { sections: Some(En1999SectionsRows::modification(&payload.section_id, En1999SectionsPatch { elements: Some(En1999SectionsElementsRows::modification(&payload.element_id, En1999SectionsElementsPatch { width: Some(payload.new_width), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
