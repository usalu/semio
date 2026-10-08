//! Inverse for `insert-assessment`.
use super::InsertAssessment;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_assessment;

pub fn inverse(payload: &InsertAssessment, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::RemoveAssessment(remove_assessment::RemoveAssessment { index: payload.index.unwrap_or(usize::MAX).min(base.assessments.len()) })]

    })())
}
