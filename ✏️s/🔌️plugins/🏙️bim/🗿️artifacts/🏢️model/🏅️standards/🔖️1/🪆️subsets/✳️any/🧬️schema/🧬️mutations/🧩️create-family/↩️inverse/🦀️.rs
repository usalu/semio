//! ↩️ Inverse of `CreateFamily`: the concrete `DeleteFamily` of the id it created, none when the id was already taken.

use super::super::delete_family::DeleteFamily;
use super::CreateFamily;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CreateFamily, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.families.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteFamily(DeleteFamily { id: payload.id.clone() })]
}
