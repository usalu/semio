//! 🔺️ Sparse `FlowDiff` construction for `disconnect-widgets` — a real synapse removal (never a
//! whole-snapshot capture).
use crate::artifacts::flow::schema::diff::text::{FlowDiff, FlowSynapsesDelta};
use crate::artifacts::flow::FlowSnapshot;

use super::mutation::DisconnectWidgets;

pub fn diff(payload: &DisconnectWidgets, _base: &FlowSnapshot) -> FlowDiff {
    FlowDiff { synapses: Some(FlowSynapsesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
