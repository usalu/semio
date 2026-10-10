//! ↩️ Inverse of `CreateClashSet`: the concrete `DeleteClashSet` of the id it created, none when the id was already taken.

use super::super::delete_clash_set::DeleteClashSet;
use super::CreateClashSet;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateClashSet, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.clash_sets.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteClashSet(DeleteClashSet { id: payload.id.clone() })]
}
