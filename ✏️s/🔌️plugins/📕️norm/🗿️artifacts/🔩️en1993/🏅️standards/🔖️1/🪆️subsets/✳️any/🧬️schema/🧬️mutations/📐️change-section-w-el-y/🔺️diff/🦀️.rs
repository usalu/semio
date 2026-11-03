//! 🔺️ `change-section-w-el-y` diff: patches `w_el_y` of the addressed section row.

use crate::change_section_w_el_y::ChangeSectionWElY;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SectionDelta, En1993SectionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSectionWElY, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_w_el_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "section", Vec::<String>::new());
    };
    if row.w_el_y == payload.new_w_el_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { sections: En1993SectionDelta::modification(payload.id.clone(), En1993SectionPatch { w_el_y: Some(payload.new_w_el_y), ..Default::default() }), ..Default::default() })
}
