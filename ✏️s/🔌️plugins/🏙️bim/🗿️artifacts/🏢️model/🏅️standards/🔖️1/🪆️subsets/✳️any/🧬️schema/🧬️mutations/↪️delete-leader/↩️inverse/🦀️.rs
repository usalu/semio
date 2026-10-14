//! ↩️ Inverse of `DeleteLeader`: the concrete `CreateLeader` carrying the full removed record, in storage order through the shared cascade.

use super::super::cascade;
use super::DeleteLeader;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &DeleteLeader, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
