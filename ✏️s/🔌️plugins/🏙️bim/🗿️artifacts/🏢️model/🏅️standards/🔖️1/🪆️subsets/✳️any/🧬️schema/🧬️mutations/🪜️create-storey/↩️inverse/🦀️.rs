//! ↩️ Inverse of `CreateStorey`: the concrete `DeleteStorey` of the id it created, none when the id was already taken.

use super::super::delete_storey::DeleteStorey;
use super::CreateStorey;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateStorey, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.storeys.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteStorey(DeleteStorey { id: payload.id.clone() })]
}
