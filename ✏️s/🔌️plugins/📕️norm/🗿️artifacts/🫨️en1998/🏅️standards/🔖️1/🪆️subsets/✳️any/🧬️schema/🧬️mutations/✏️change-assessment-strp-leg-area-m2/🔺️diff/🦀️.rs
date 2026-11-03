//! ✏️ `change-assessment-strp-leg-area-m2` diff: patches `stirrup_leg_area_m2` of the addressed assessment row.

use crate::change_assessment_strp_leg_area_m2::ChangeAssessmentStrpLegAreaM2;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentStrpLegAreaM2, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_stirrup_leg_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.stirrup_leg_area_m2 == payload.new_stirrup_leg_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { stirrup_leg_area_m2: Some(payload.new_stirrup_leg_area_m2), ..Default::default() }), ..Default::default() })
}
