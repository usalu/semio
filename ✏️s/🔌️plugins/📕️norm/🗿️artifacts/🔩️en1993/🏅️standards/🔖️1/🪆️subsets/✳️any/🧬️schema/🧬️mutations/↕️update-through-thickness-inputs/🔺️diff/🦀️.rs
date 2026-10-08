//! ↕️ `update-through-thickness-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateThroughThicknessInputs;
use crate::diff::{En1993Diff, En1993SectionDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateThroughThicknessInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.sections.iter().position(|row| row.id == payload.section.id) {
        Some(index) if base.sections[index] == payload.section => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993SectionDelta::removal(&base.sections, index);
            replacement.absorb(En1993SectionDelta::insertion(index, payload.section.clone()));
            replacement
        }
        None => En1993SectionDelta::insertion(base.sections.len(), payload.section.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { sections: delta, ..Default::default() })
}
