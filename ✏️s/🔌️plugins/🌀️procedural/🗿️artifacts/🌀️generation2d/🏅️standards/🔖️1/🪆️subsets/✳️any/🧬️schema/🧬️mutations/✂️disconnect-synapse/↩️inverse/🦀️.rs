//! ↩️ Inverse for `DisconnectSynapse` — reconnects the removed edge at its captured BASE index, or
//! a no-op (`Vec::new()`) when the id was already absent.

use crate::artifacts::generation2d::mutations::{connect_synapse, Generation2dMutation};
use crate::artifacts::generation2d::Generation2dSnapshot;

pub fn inverse(payload: &super::DisconnectSynapse, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.fixture.synapses.iter().position(|synapse| synapse.id == payload.id) {
        Some(index) => vec![connect_synapse(index, base.fixture.synapses[index].clone())],
        None => Vec::new(),
    }
}
