//! ↩️ `specify-dhw-system` inverse.

use crate::mutations::specify_dhw_system::SpecifyDhwSystem;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &SpecifyDhwSystem, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::SpecifyDhwSystem(SpecifyDhwSystem { new_dhw: base.dhw.clone() })]
}
