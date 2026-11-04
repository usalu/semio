//! ✏️ `change-assessment-analysis-method` diff: patches `analysis_method` of the addressed assessment row.

use crate::change_assessment_analysis_method::ChangeAssessmentAnalysisMethod;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998AssessmentPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentAnalysisMethod, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.assessments.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("assessment {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.analysis_method == payload.new_analysis_method_choice {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The assessment already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&payload.id, En1998AssessmentPatch { analysis_method: Some(payload.new_analysis_method_choice), ..Default::default() }), ..Default::default() })
}
