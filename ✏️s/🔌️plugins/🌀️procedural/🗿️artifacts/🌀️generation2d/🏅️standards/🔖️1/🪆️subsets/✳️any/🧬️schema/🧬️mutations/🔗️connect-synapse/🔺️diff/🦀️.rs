//! 🔺️ Sparse diff builder for `ConnectSynapse` — a real id-keyed upsert into the fixture's synapse
//! collection helper (never a whole-snapshot capture).

use crate::standards::v1::subsets::any::schema::diff::{Generation2dDiff, Generation2dSynapsesDelta};
use crate::{widget_id, Generation2dSnapshot};

pub fn diff(payload: &super::ConnectSynapse, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let synapse = &payload.synapse;
    if base.host_snapshot.synapses.iter().any(|entry| entry.id == synapse.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A synapse with id \"{}\" already exists.", synapse.id), [synapse.id.clone()]);
    }
    if !base.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == synapse.from) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Source widget \"{}\" does not exist.", synapse.from), [synapse.from.clone()]);
    }
    if !base.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == synapse.to) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Target widget \"{}\" does not exist.", synapse.to), [synapse.to.clone()]);
    }
    if base.host_snapshot.synapses.iter().any(|entry| entry.from == synapse.from && entry.from_port == synapse.from_port && entry.to == synapse.to && entry.to_port == synapse.to_port) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("\"{}\" is already connected to \"{}\"; parallel synapses are not allowed.", synapse.from, synapse.to));
    }
    let reordered = ((payload.index) < base.host_snapshot.synapses.len()).then(|| { let mut order: Vec<String> = base.host_snapshot.synapses.iter().map(|entry| entry.id.to_string()).collect(); order.insert(payload.index, synapse.id.to_string()); order });
    protocol::MutationOutcome::new(Generation2dDiff { synapses: Some(Generation2dSynapsesDelta { added: vec![synapse.clone()], reordered, ..Default::default() }), ..Default::default() })
}
