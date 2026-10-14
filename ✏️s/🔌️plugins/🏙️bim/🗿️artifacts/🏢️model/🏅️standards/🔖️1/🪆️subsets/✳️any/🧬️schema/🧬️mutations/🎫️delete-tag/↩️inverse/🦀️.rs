//! ↩️ Inverse of `DeleteTag`: the concrete `CreateTag` carrying the full removed record, in storage order through the shared cascade.

use super::super::cascade;
use super::DeleteTag;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &DeleteTag, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
