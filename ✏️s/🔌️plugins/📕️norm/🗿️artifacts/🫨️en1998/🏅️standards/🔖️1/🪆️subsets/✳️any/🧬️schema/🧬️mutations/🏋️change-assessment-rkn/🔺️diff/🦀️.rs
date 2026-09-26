//! Diff for `change-assessment-r-k-n`.
use super::ChangeAssessmentRKN;
use crate::{En1998Diff, En1998Snapshot};

pub fn diff(payload: &ChangeAssessmentRKN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let mut assessments = base.assessments.clone();
    let Some(a) = assessments.get_mut(payload.index) else { return protocol::MutationOutcome::error("mutation.target-missing", "assessment", Vec::<String>::new()); };
    a.r_k_n = payload.new_r_k_n;
    protocol::MutationOutcome::new(En1998Diff { assessments: Some(assessments), ..Default::default() })
}
