//! 🔺️ Sparse diff builder for `ConnectSynapse` — a real id-keyed upsert into the fixture's synapse
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::generation2d::diff::{diff_fixture_from_helpers, LayoutDiff, Generation2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::generation2d::{widget_id, Generation2dSnapshot};

pub fn diff(payload: &super::ConnectSynapse, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let synapse = &payload.synapse;
    if base.fixture.synapses.iter().any(|entry| entry.id == synapse.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A synapse with id \"{}\" already exists.", synapse.id), [synapse.id.clone()]);
    }
    if !base.fixture.widgets.iter().any(|widget| widget_id(widget) == synapse.from) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Source widget \"{}\" does not exist.", synapse.from), [synapse.from.clone()]);
    }
    if !base.fixture.widgets.iter().any(|widget| widget_id(widget) == synapse.to) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Target widget \"{}\" does not exist.", synapse.to), [synapse.to.clone()]);
    }
    if base.fixture.synapses.iter().any(|entry| entry.from == synapse.from && entry.from_port == synapse.from_port && entry.to == synapse.to && entry.to_port == synapse.to_port) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("\"{}\" is already connected to \"{}\"; parallel synapses are not allowed.", synapse.from, synapse.to));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![], set: vec![(payload.index, synapse.clone())] }, LayoutDiff::default(), None, None))
}
