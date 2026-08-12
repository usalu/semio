//! 🔺️ `connect-synapse` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::connect_synapse::mutation::ConnectSynapse;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta for one new synapse edge.
pub fn diff(payload: &ConnectSynapse, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(
        base,
        WidgetsDiff::default(),
        SynapsesDiff { removed: vec![], set: vec![(payload.index, payload.synapse.clone())] },
        LayoutDiff::default(),
        None,
        None,
    )
}
