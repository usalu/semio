//! ↩️ Inverse of `CreateSlab`: the concrete `DeleteSlab` of the id it created, none when the id was already taken.

use super::super::delete_slab::DeleteSlab;
use super::CreateSlab;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSlab, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.slabs.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSlab(DeleteSlab { id: payload.id.clone() })]
}
