//! Inverse for `insert-assessment`.
use super::InsertAssessment;
use crate::{En1998Mutation, En1998Snapshot};

pub fn inverse(_payload: &InsertAssessment, _base: &En1998Snapshot) -> Vec<En1998Mutation> {
    Vec::new()
}
