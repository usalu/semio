//! ↩️ Inverse of `CreateTag`: the concrete `DeleteTag` of the id it created, none when the id was already taken.

use super::super::delete_tag::DeleteTag;
use super::CreateTag;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};

pub fn inverse(payload: &CreateTag, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.tags.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteTag(DeleteTag { id: payload.id.clone() })]
}
