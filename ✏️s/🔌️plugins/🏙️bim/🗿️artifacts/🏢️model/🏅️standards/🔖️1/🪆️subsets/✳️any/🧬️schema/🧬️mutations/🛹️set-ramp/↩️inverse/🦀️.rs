//! ↩️ Inverse of `SetRamp`: an absolute `SetRamp` restoring the base value of exactly the fields the forward really changes, none when the ramp is absent or nothing changes.

use super::SetRamp;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetRamp, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.ramps.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetRamp(SetRamp::from_patch(payload.id.clone(), restore))]
}
