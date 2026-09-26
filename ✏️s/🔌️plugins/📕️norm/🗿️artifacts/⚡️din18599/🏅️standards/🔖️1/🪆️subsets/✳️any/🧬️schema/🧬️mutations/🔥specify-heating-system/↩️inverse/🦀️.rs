//! ↩️ `specify-heating-system` inverse.

use crate::mutations::specify_heating_system::SpecifyHeatingSystem;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &SpecifyHeatingSystem, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::SpecifyHeatingSystem(SpecifyHeatingSystem { new_heating: base.heating.clone() })]
}
