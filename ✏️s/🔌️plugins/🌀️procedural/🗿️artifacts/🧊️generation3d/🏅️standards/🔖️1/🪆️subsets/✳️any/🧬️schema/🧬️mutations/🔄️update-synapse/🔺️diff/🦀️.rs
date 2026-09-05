//! 🔺️ `update-synapse` sparse diff construction.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::generation3d::mutations::synapse_index;
use crate::artifacts::generation3d::mutations::update_synapse::UpdateSynapse;
use crate::artifacts::generation3d::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta replacing one existing synapse's ports. The index is
/// irrelevant here — `apply_synapses_diff` resolves an existing entry by id first.
pub fn diff(payload: &UpdateSynapse, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = &payload.synapse.id;
    let Some(index) = synapse_index(&base.fixture, id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{id}\" does not exist."), [id.clone()]);
    };
    if base.fixture.synapses[index] == payload.synapse {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warn("mutation.no-op", format!("Synapse \"{id}\" is already in the requested state."));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![], set: vec![(0, payload.synapse.clone())] }, LayoutDiff::default(), None, None))
}
