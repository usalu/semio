//! ↩️ Inverse of `CreateComponent`: the concrete `DeleteComponent` of the id it created, none when the id was already taken.

use super::super::delete_component::DeleteComponent;
use super::CreateComponent;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateComponent, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.components.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteComponent(DeleteComponent { id: payload.id.clone() })]
}
