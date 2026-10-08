//! 🏋️ `change-assessment-rkn` diff — patches the one field of the row at the index; a missing row is a `mutation.target-missing`.

use super::ChangeAssessmentRKN;
use crate::diff::{En1998Diff, En1998AssessmentDelta, En1998AssessmentPatch};
use crate::En1998Snapshot;

pub fn diff(payload: &ChangeAssessmentRKN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(row) = base.assessments.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "assessment", Vec::<String>::new());
    };
    let patch = En1998AssessmentPatch { r_k_n: Some(payload.new_r_k_n), ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { assessments: En1998AssessmentDelta::modification(&row.id, patch), ..Default::default() })
}
