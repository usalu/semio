//! ↩️ Inverse of `DeleteTextNote`: the concrete `CreateTextNote` carrying the full removed record, in storage order through the shared cascade.

use super::super::cascade;
use super::DeleteTextNote;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &DeleteTextNote, base: &ModelSnapshot) -> Vec<ModelMutation> {
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
