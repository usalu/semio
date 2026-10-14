//! ↩️ Inverse of `CreateRailing`: the concrete `DeleteRailing` of the id it created, none when the id was already taken.

use super::super::delete_railing::DeleteRailing;
use super::CreateRailing;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CreateRailing, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.railings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteRailing(DeleteRailing { id: payload.id.clone() })]
}
