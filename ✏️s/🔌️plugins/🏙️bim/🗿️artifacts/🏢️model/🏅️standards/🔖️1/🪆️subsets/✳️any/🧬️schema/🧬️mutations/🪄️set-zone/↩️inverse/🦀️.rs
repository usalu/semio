//! ↩️ Inverse of `SetZone`: an absolute `SetZone` restoring the base value of exactly the fields the forward really changes, none when the zone is absent or nothing changes.

use super::SetZone;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetZone, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.zones.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetZone(SetZone::from_patch(payload.id.clone(), restore))]
}
