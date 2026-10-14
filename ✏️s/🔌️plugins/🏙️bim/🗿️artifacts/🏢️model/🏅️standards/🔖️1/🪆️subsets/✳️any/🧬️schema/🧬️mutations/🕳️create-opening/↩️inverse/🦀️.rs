//! ↩️ Inverse of `CreateOpening`: the concrete `DeleteOpening` of the id it created, none when the id was already taken.

use super::super::delete_opening::DeleteOpening;
use super::CreateOpening;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CreateOpening, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.openings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteOpening(DeleteOpening { id: payload.id.clone() })]
}
