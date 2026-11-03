//! ➕️ `insert-parametric-section` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_parametric_section::InsertParametricSection;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ParametricSectionDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertParametricSection, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.parametric_sections.iter().any(|existing| existing.id == payload.parametric_section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Parametric section id {} already exists.", payload.parametric_section.id), [payload.parametric_section.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.parametric_sections.len());
    protocol::MutationOutcome::new(En1993Diff { parametric_sections: En1993ParametricSectionDelta::insertion(index, payload.parametric_section.clone()), ..Default::default() })
}
