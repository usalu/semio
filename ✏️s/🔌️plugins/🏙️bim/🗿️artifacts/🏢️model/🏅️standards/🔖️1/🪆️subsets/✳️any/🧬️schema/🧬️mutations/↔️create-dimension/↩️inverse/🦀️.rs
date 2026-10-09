//! ↩️ Inverse of `CreateDimension`: the concrete `DeleteDimension` of the id it created, none when the id was already taken.

use super::super::delete_dimension::DeleteDimension;
use super::CreateDimension;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateDimension, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.dimensions.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteDimension(DeleteDimension { id: payload.id.clone() })]
}
