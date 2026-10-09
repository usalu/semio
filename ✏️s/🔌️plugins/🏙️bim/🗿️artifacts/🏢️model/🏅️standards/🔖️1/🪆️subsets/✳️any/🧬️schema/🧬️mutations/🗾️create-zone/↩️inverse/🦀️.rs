//! ↩️ Inverse of `CreateZone`: the concrete `DeleteZone` of the id it created, none when the id was already taken.

use super::super::delete_zone::DeleteZone;
use super::super::elements;
use super::CreateZone;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateZone, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if elements::taken(base, &payload.id).is_some() {
        return Vec::new();
    }
    vec![ModelMutation::DeleteZone(DeleteZone { id: payload.id.clone() })]
}
