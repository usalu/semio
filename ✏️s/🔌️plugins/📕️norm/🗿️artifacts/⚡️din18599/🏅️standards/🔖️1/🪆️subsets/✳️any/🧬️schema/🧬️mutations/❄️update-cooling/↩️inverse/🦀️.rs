//! ↩️ `update-cooling` inverse.

use crate::mutations::update_cooling::UpdateCooling;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &UpdateCooling, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::UpdateCooling(UpdateCooling { new_cooling: base.cooling.clone() })]
}
