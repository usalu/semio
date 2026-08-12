//! 🔺️ `move-widget` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::set_layout::mutation::MoveWidget;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta upserting one widget's position.
pub fn diff(payload: &MoveWidget, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(
        base,
        WidgetsDiff::default(),
        SynapsesDiff::default(),
        LayoutDiff { removed: vec![], set: vec![(payload.id.clone(), payload.layout.clone())] },
        None,
        None,
    )
}
