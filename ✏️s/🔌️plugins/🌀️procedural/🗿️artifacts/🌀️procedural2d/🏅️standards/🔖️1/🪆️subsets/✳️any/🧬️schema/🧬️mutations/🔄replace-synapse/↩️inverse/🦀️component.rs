//! ↩️ Inverse for `ReplaceSynapse`, reconstructed from BASE.
use super::mutation::ReplaceSynapse;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::mutations::{replace_synapse, synapse_index};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSynapse, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    match synapse_index(&base.fixture, &payload.synapse.id) {
        Some(index) => vec![replace_synapse(base.fixture.synapses[index].clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
