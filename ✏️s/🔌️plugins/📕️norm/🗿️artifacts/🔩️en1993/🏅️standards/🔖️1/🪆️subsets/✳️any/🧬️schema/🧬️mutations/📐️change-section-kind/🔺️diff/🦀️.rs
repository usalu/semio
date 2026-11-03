//! 🔺️ `change-section-kind` diff: patches `kind` of the addressed section row.

use crate::change_section_kind::ChangeSectionKind;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993SectionDelta, En1993SectionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeSectionKind, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "section", Vec::<String>::new());
    };
    if row.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { sections: En1993SectionDelta::modification(payload.id.clone(), En1993SectionPatch { kind: Some(payload.new_kind), ..Default::default() }), ..Default::default() })
}
