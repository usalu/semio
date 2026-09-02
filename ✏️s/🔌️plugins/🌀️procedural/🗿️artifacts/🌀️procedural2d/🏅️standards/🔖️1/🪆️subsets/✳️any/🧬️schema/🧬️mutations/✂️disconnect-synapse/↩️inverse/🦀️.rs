//! ↩️ Inverse for `DisconnectSynapse` — reconnects the removed edge at its captured BASE index, or
//! a no-op (`Vec::new()`) when the id was already absent.

use crate::artifacts::procedural2d::mutations::{connect_synapse, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn inverse(payload: &super::DisconnectSynapse, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    match base.fixture.synapses.iter().position(|synapse| synapse.id == payload.id) {
        Some(index) => vec![connect_synapse(index, base.fixture.synapses[index].clone())],
        None => Vec::new(),
    }
}
