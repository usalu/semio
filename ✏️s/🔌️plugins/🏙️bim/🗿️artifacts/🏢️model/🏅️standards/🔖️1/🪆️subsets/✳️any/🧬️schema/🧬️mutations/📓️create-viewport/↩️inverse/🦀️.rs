//! ↩️ Inverse of `CreateViewport`: the concrete `DeleteViewport` of the id it created, none when the id was already taken.

use super::super::delete_viewport::DeleteViewport;
use super::CreateViewport;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateViewport, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.viewports.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteViewport(DeleteViewport { id: payload.id.clone() })]
}
