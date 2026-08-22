//! 🔺️ `update-synapse` sparse diff construction.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::synapse_index;
use crate::artifacts::procedural3d::mutations::update_synapse::mutation::UpdateSynapse;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta replacing one existing synapse's ports. The index is
/// irrelevant here — `apply_synapses_diff` resolves an existing entry by id first.
pub fn diff(payload: &UpdateSynapse, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    let id = &payload.synapse.id;
    let Some(index) = synapse_index(&base.fixture, id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse \"{id}\" does not exist."), [id.clone()]);
    };
    if base.fixture.synapses[index] == payload.synapse {
        return protocol::MutationOutcome::new(Procedural3dDiff::default()).warn("mutation.no-op", format!("Synapse \"{id}\" is already in the requested state."));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![], set: vec![(0, payload.synapse.clone())] }, LayoutDiff::default(), None, None))
}
