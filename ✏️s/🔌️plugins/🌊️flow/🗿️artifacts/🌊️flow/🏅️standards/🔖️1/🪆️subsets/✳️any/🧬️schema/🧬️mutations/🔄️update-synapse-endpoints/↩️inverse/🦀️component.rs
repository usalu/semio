//! ↩️ Undo mutation for `update-synapse-endpoints`: restore the synapse's prior `base` endpoints.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::{flow_working_scene, FlowSnapshot};

use super::mutation::UpdateSynapseEndpoints;

pub async fn inverse(payload: &UpdateSynapseEndpoints, base: &FlowSnapshot) -> Vec<FlowMutation> {
    let scene = flow_working_scene(base);
    match scene.synapses.iter().find(|synapse| synapse.id == payload.id) {
        Some(previous) => vec![FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: payload.id.clone(), from: previous.from.clone(), from_port: previous.from_port.clone(), to: previous.to.clone(), to_port: previous.to_port.clone() })],
        None => Vec::new(),
    }
}
