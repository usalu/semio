//! ↩️ Inverse of `CreateCeiling`: the concrete `DeleteCeiling` of the id it created, none when the id was already taken.

use super::super::delete_ceiling::DeleteCeiling;
use super::CreateCeiling;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateCeiling, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.ceilings.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteCeiling(DeleteCeiling { id: payload.id.clone() })]
}
