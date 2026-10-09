//! ↩️ Inverse of `CreateRamp`: the concrete `DeleteRamp` of the id it created, none when the id was already taken.

use super::super::delete_ramp::DeleteRamp;
use super::CreateRamp;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateRamp, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.ramps.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteRamp(DeleteRamp { id: payload.id.clone() })]
}
