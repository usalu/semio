//! ↩️ Inverse of `SetBeamType`: an absolute `SetBeamType` restoring the base value of exactly the fields the forward really changes, none when the beam type is absent or nothing changes.

use super::SetBeamType;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetBeamType, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.beam_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetBeamType(SetBeamType::from_patch(payload.id.clone(), restore))]
}
