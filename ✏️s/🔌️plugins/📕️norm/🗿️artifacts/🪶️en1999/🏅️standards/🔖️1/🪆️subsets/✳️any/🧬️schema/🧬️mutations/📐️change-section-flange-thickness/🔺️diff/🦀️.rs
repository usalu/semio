//! 🔺️ `change-section-flange-thickness` diff: patches `flange_thickness` of the addressed section row.

use crate::change_section_flange_thickness::ChangeSectionFlangeThickness;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999SectionsPatch, En1999SectionsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSectionFlangeThickness, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_flange_thickness.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.sections.iter().find(|row| row.id == payload.section_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "section", Vec::<String>::new());
    };
    if r0.flange_thickness == payload.new_flange_thickness {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { sections: Some(En1999SectionsRows::modification(&payload.section_id, En1999SectionsPatch { flange_thickness: Some(payload.new_flange_thickness), ..Default::default() })), ..Default::default() })
}
