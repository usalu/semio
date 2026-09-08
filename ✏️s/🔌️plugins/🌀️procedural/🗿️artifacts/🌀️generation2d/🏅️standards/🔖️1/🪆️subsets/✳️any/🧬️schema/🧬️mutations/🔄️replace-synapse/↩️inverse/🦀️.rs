//! ↩️ Inverse for `ReplaceSynapse`, reconstructed from BASE.
use super::ReplaceSynapse;
use crate::mutations::Generation2dMutation;
use crate::mutations::{replace_synapse, synapse_index};
use crate::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSynapse, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match synapse_index(&base.fixture, &payload.synapse.id) {
        Some(index) => vec![replace_synapse(base.fixture.synapses[index].clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
