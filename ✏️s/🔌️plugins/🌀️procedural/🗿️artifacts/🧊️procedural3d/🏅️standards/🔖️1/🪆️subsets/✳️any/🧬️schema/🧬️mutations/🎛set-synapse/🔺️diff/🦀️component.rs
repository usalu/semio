//! 🔺️ `update-synapse` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::set_synapse::mutation::UpdateSynapse;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta replacing one existing synapse's ports. The index is
/// irrelevant here — `apply_synapses_diff` resolves an existing entry by id first.
pub fn diff(payload: &UpdateSynapse, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(
        base,
        WidgetsDiff::default(),
        SynapsesDiff { removed: vec![], set: vec![(0, payload.synapse.clone())] },
        LayoutDiff::default(),
        None,
        None,
    )
}
