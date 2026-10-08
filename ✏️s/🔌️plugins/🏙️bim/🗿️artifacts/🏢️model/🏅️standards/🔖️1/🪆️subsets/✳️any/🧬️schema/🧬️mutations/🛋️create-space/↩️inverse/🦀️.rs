//! ↩️ Inverse of `CreateSpace`: the concrete `DeleteSpace` of the id it created, none when the id was already taken.

use super::super::delete_space::DeleteSpace;
use super::CreateSpace;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSpace, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.spaces.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSpace(DeleteSpace { id: payload.id.clone() })]
}
