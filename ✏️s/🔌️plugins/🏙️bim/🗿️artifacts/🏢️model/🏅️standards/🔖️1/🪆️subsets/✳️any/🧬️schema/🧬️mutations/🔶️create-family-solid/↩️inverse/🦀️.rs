//! ↩️ Inverse of `CreateFamilySolid`: the concrete `DeleteFamilySolid` of the id it created, none when the id was already taken.

use super::super::delete_family_solid::DeleteFamilySolid;
use super::CreateFamilySolid;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateFamilySolid, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.family_solids.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteFamilySolid(DeleteFamilySolid { id: payload.id.clone() })]
}
