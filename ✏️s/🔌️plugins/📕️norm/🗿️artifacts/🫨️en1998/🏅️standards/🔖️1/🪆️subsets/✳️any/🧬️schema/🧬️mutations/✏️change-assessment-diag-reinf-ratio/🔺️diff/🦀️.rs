//! ✏️ `change-assessment-diag-reinf-ratio` diff: patches `diagonal_reinforcement_ratio` of the addressed assessment row.

use crate::change_assessment_diag_reinf_ratio::ChangeAssessmentDiagReinfRatio;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentDiagReinfRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_diagonal_reinforcement_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.diagonal_reinforcement_ratio == payload.new_diagonal_reinforcement_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { diagonal_reinforcement_ratio: Some(payload.new_diagonal_reinforcement_ratio), ..Default::default() }), ..Default::default() })
}
