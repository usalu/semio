//! ↩️ Inverse of `SetColumnTilt`: an absolute `SetColumnTilt` back to the base tilt (plumb when none), none when the column is absent.

use super::SetColumnTilt;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetColumnTilt, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.columns.get(&payload.id) {
        Some(column) => vec![ModelMutation::SetColumnTilt(SetColumnTilt { id: payload.id.clone(), tilt: column.tilt })],
        None => Vec::new(),
    }
}
