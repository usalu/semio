//! ↩️ Inverse of `SetTypeThermalData`: an absolute `SetTypeThermalData` restoring the base value of exactly the fields the forward really changes, none when the type is absent or nothing changes.

use super::SetTypeThermalData;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetTypeThermalData, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let restore = if let Some(record) = base.window_types.get(&payload.id) {
        let patch = payload.window_patch().minimal(record).restoring(record);
        SetTypeThermalData { id: payload.id.clone(), u_value: patch.u_value, g_value: patch.g_value, frame_fraction: patch.frame_fraction }
    } else if let Some(record) = base.curtain_wall_types.get(&payload.id) {
        let patch = payload.curtain_patch().minimal(record).restoring(record);
        SetTypeThermalData { id: payload.id.clone(), u_value: patch.u_value, g_value: patch.g_value, frame_fraction: patch.frame_fraction }
    } else if let Some(record) = base.door_types.get(&payload.id) {
        let patch = payload.door_patch().minimal(record).restoring(record);
        SetTypeThermalData { id: payload.id.clone(), u_value: patch.u_value, g_value: None, frame_fraction: None }
    } else {
        return Vec::new();
    };
    if restore.u_value.is_none() && restore.g_value.is_none() && restore.frame_fraction.is_none() {
        return Vec::new();
    }
    vec![ModelMutation::SetTypeThermalData(restore)]
}
