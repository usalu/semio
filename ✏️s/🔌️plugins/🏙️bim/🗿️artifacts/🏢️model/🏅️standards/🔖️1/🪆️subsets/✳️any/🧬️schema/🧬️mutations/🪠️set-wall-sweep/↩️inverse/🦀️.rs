//! ↩️ Inverse of `SetWallSweep`: an absolute `SetWallSweep` restoring the base value of exactly the fields the forward really changes, none when the sweep is absent or nothing changes.

use super::SetWallSweep;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetWallSweep, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.wall_sweeps.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetWallSweep(SetWallSweep::from_patch(payload.id.clone(), restore))]
}
