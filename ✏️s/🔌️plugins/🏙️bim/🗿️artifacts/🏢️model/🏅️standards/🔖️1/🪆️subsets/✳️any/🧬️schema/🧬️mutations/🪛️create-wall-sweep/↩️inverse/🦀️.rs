//! ↩️ Inverse of `CreateWallSweep`: the concrete `DeleteWallSweep` of the id it created, none when the id was already taken.

use super::super::delete_wall_sweep::DeleteWallSweep;
use super::CreateWallSweep;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateWallSweep, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.wall_sweeps.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteWallSweep(DeleteWallSweep { id: payload.id.clone() })]
}
