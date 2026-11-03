//! ✏️ `change-assessment-gamma-el` diff: patches `gamma_el` of the addressed assessment row.

use crate::change_assessment_gamma_el::ChangeAssessmentGammaEl;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentGammaEl, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_gamma_el.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.gamma_el == payload.new_gamma_el {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { gamma_el: Some(payload.new_gamma_el), ..Default::default() }), ..Default::default() })
}
