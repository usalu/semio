//! 🔺️ Sparse `FlowDiff` construction for `update-synapse-endpoints` — a real whole-endpoints patch
//! entry construction (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{FlowDiff, FlowSynapsePatchEntry, FlowSynapsesDelta};
use crate::artifacts::flow::FlowSnapshot;
use flow::SynapseSpec;

use super::mutation::UpdateSynapseEndpoints;

pub fn diff(payload: &UpdateSynapseEndpoints, _base: &FlowSnapshot) -> FlowDiff {
    let synapse = SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() };
    let delta = FlowSynapsesDelta { patched: vec![FlowSynapsePatchEntry { id: payload.id.clone(), patch: synapse }], ..Default::default() };
    FlowDiff { synapses: Some(delta), ..Default::default() }
}
