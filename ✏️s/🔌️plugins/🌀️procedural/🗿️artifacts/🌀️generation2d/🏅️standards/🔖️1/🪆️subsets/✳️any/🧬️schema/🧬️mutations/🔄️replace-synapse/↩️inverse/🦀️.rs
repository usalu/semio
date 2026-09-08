//! ↩️ Inverse for `ReplaceSynapse`, reconstructed from BASE.
use super::ReplaceSynapse;
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{replace_synapse, synapse_index};
use crate::Generation2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceSynapse, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match synapse_index(&base.fixture, &payload.synapse.id) {
        Some(index) => vec![replace_synapse(base.fixture.synapses[index].clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
