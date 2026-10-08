//! ↩️ Inverse of `CreateWindowType`: the concrete `DeleteWindowType` of the id it created, none when the id was already taken.

use super::super::delete_window_type::DeleteWindowType;
use super::CreateWindowType;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateWindowType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.window_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteWindowType(DeleteWindowType { id: payload.id.clone() })]
}
