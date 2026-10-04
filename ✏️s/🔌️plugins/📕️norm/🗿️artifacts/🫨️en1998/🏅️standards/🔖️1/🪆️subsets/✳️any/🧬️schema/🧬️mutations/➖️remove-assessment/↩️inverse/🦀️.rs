//! Inverse for `remove-assessment`.
use super::RemoveAssessment;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_assessment;

pub fn inverse(payload: &RemoveAssessment, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.assessments.get(payload.index) {
        Some(item) => vec![En1998Mutation::InsertAssessment(insert_assessment::InsertAssessment { index: payload.index, assessment: item.clone() })],
        None => Vec::new(),
    }

    })())
}
