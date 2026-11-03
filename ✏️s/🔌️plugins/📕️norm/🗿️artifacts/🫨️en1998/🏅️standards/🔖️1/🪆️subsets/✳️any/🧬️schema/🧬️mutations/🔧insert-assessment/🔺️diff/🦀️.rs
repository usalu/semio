//! 🔧 `insert-assessment` diff: inserts the assessment row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_assessment::InsertAssessment;
use semio_s_artifact_norm_en1998_diff::{En1998AssessmentDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertAssessment, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.assessments.iter().any(|existing| existing.id == payload.assessment.id) {
        let key = payload.assessment.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("assessment {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.assessments.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::insertion(index, payload.assessment.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the assessment list; inserted at {index}.", payload.index.unwrap_or(index)))
}
