//! Inverse for `change-assessment-r-k-n`.
use super::ChangeAssessmentRKN;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(payload: &ChangeAssessmentRKN, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.assessments.get(payload.index) {
        Some(a) => vec![En1998Mutation::ChangeAssessmentRKN(ChangeAssessmentRKN { index: payload.index, new_r_k_n: a.r_k_n })],
        None => Vec::new(),
    }
}
