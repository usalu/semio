//! 🔺️ `change-section-r` diff: patches `r` of the addressed section row.

use crate::change_section_r::ChangeSectionR;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SectionDelta, En1993SectionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSectionR, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_r.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "section", Vec::<String>::new());
    };
    if row.r == payload.new_r {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { sections: En1993SectionDelta::modification(payload.id.clone(), En1993SectionPatch { r: Some(payload.new_r), ..Default::default() }), ..Default::default() })
}
