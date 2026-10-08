//! ↩️ Inverse of `CreateBuilding`: the concrete `DeleteBuilding` of the id it created, none when the id was already taken.

use super::super::delete_building::DeleteBuilding;
use super::CreateBuilding;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateBuilding, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.buildings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteBuilding(DeleteBuilding { id: payload.id.clone() })]
}
