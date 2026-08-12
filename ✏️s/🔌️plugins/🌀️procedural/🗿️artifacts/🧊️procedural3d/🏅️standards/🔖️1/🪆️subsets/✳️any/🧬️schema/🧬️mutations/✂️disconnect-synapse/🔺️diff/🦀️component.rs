//! 🔺️ `disconnect-synapse` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::disconnect_synapse::mutation::DisconnectSynapse;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta severing one synapse edge by id.
pub fn diff(payload: &DisconnectSynapse, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(
        base,
        WidgetsDiff::default(),
        SynapsesDiff { removed: vec![payload.id.clone()], set: vec![] },
        LayoutDiff::default(),
        None,
        None,
    )
}
