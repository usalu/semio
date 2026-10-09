//! ↩️ Inverse of `SetBeam`: an absolute `SetBeam` restoring the base value of exactly the fields the forward really changes, none when the beam is absent or nothing changes.

use super::SetBeam;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetBeam, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.beams.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetBeam(SetBeam::from_patch(payload.id.clone(), restore))]
}
