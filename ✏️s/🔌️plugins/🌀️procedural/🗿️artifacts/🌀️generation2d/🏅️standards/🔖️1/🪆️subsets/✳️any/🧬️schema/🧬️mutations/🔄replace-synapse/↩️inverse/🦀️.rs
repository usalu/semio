//! ↩️ Inverse for `ReplaceSynapse`, reconstructed from BASE.
use super::ReplaceSynapse;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::mutations::{replace_synapse, synapse_index};
use crate::artifacts::generation2d::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSynapse, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match synapse_index(&base.fixture, &payload.synapse.id) {
        Some(index) => vec![replace_synapse(base.fixture.synapses[index].clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
