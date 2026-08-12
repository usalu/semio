//! 🔺️ Sparse `FlowDiff` construction for `connect-widgets` — a real append-only synapse insert
//! (never a whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{FlowDiff, FlowSynapsesDelta};
use crate::artifacts::flow::FlowSnapshot;
use flow::SynapseSpec;

use super::mutation::ConnectWidgets;

pub fn diff(payload: &ConnectWidgets, _base: &FlowSnapshot) -> FlowDiff {
    let synapse = SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() };
    FlowDiff { synapses: Some(FlowSynapsesDelta { added: vec![synapse], ..Default::default() }), ..Default::default() }
}
