//! 🔺️ `change-param-section-id` diff: patches `section_id` of the addressed parametric section row.

use crate::change_param_section_id::ChangeParamSectionId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ParametricSectionDelta, En1993ParametricSectionPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeParamSectionId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.parametric_sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "param-section", Vec::<String>::new());
    };
    if row.section_id == payload.new_section_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The parametric section already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { parametric_sections: En1993ParametricSectionDelta::modification(payload.id.clone(), En1993ParametricSectionPatch { section_id: Some(payload.new_section_id.clone()), ..Default::default() }), ..Default::default() })
}
