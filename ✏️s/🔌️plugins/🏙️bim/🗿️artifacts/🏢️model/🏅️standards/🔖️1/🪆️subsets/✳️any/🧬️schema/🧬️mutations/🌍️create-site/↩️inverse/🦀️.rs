//! ↩️ Inverse of `CreateSite`: the concrete `DeleteSite` of the id it created, none when the id was already taken.

use super::CreateSite;
use super::super::delete_site::DeleteSite;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateSite, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.sites.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteSite(DeleteSite { id: payload.id.clone() })]
}
