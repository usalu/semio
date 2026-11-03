//! ✏️ `change-assessment-bar-diameter-mm` diff: patches `bar_diameter_mm` of the addressed assessment row.

use crate::change_assessment_bar_diameter_mm::ChangeAssessmentBarDiameterMm;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentBarDiameterMm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_bar_diameter_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.bar_diameter_mm == payload.new_bar_diameter_mm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { bar_diameter_mm: Some(payload.new_bar_diameter_mm), ..Default::default() }), ..Default::default() })
}
