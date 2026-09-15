//! ↩️ Inverse for `DisconnectSynapse` — reconnects the removed edge at its captured BASE index, or
//! a no-op (`Vec::new()`) when the id was already absent.

use crate::standards::v1::subsets::any::schema::mutations::{connect_synapse, Generation2dMutation};
use crate::Generation2dSnapshot;

pub fn inverse(payload: &super::DisconnectSynapse, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.host_document.synapses.iter().position(|synapse| synapse.id == payload.id) {
        Some(index) => vec![connect_synapse(index, base.host_document.synapses[index].clone())],
        None => Vec::new(),
    }
}
