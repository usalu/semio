//! 🔺️ `change-section-iy` diff: patches `iy` of the addressed section row.

use crate::change_section_iy::ChangeSectionIy;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SectionDelta, En1993SectionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSectionIy, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_iy.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "section", Vec::<String>::new());
    };
    if row.iy == payload.new_iy {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { sections: En1993SectionDelta::modification(payload.id.clone(), En1993SectionPatch { iy: Some(payload.new_iy), ..Default::default() }), ..Default::default() })
}
