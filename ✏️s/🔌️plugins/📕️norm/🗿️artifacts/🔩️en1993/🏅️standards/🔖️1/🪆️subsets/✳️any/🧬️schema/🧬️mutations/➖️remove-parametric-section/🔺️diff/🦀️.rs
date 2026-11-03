//! ➖️ `remove-parametric-section` diff — removes the row at the index.

use crate::remove_parametric_section::RemoveParametricSection;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ParametricSectionDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveParametricSection, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.parametric_sections.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("parametric-section index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { parametric_sections: En1993ParametricSectionDelta::removal(&base.parametric_sections, payload.index), ..Default::default() })
}
