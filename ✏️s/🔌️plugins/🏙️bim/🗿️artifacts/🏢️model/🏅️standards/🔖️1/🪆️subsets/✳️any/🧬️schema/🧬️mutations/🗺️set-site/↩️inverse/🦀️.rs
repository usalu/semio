//! ↩️ Inverse of `SetSite`: an absolute `SetSite` restoring the base value of exactly the fields the forward really changes, none when the site is absent or nothing changes.

use super::SetSite;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetSite, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.sites.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetSite(SetSite::from_patch(payload.id.clone(), restore))]
}
