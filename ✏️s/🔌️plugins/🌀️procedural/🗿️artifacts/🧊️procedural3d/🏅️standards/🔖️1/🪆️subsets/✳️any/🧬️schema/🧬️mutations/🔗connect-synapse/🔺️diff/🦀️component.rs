//! 🔺️ `connect-synapse` sparse diff construction.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::connect_synapse::mutation::ConnectSynapse;
use crate::artifacts::procedural3d::mutations::{synapse_index, widget_index};
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta for one new synapse edge.
pub fn diff(payload: &ConnectSynapse, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    let id = &payload.synapse.id;
    if synapse_index(&base.fixture, id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A synapse with id \"{id}\" already exists."), [id.clone()]);
    }
    if widget_index(&base.fixture, &payload.synapse.from).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse endpoint widget \"{}\" does not exist.", payload.synapse.from), [payload.synapse.from.clone()]);
    }
    if widget_index(&base.fixture, &payload.synapse.to).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Synapse endpoint widget \"{}\" does not exist.", payload.synapse.to), [payload.synapse.to.clone()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![], set: vec![(payload.index, payload.synapse.clone())] }, LayoutDiff::default(), None, None))
}
