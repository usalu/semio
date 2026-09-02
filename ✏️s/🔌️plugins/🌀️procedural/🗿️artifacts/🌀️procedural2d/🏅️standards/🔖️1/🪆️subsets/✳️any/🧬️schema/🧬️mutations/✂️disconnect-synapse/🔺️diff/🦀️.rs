//! 🔺️ Sparse diff builder for `DisconnectSynapse` — a real id-keyed removal from the fixture's
//! synapse collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn diff(payload: &super::DisconnectSynapse, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    if !base.fixture.synapses.iter().any(|synapse| synapse.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![payload.id.clone()], set: vec![] }, LayoutDiff::default(), None, None))
}
