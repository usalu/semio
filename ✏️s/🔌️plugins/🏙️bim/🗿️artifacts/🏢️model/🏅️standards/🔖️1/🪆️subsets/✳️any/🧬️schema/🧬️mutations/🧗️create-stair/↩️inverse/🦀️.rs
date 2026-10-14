//! ↩️ Inverse of `CreateStair`: the concrete `DeleteStair` of the id it created, none when the id was already taken.

use super::super::delete_stair::DeleteStair;
use super::CreateStair;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CreateStair, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.stairs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteStair(DeleteStair { id: payload.id.clone() })]
}
