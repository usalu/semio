//! ↩️ Inverse of `CreateRoof`: the concrete `DeleteRoof` of the id it created, none when the id was already taken.

use super::super::delete_roof::DeleteRoof;
use super::CreateRoof;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateRoof, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.roofs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteRoof(DeleteRoof { id: payload.id.clone() })]
}
