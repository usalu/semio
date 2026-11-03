//! 🔺️ `change-section-fabrication` diff: patches `fabrication` of the addressed section row.

use crate::change_section_fabrication::ChangeSectionFabrication;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SectionDelta, En1993SectionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSectionFabrication, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "section", Vec::<String>::new());
    };
    if row.fabrication == payload.new_fabrication {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { sections: En1993SectionDelta::modification(payload.id.clone(), En1993SectionPatch { fabrication: Some(payload.new_fabrication), ..Default::default() }), ..Default::default() })
}
