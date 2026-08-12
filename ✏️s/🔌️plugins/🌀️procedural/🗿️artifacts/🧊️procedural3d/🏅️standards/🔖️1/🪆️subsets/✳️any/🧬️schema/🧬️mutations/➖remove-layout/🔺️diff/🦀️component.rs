//! 🔺️ `delete-widget-position` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::remove_layout::mutation::DeleteWidgetPosition;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta removing one widget's position override.
pub fn diff(payload: &DeleteWidgetPosition, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(
        base,
        WidgetsDiff::default(),
        SynapsesDiff::default(),
        LayoutDiff { removed: vec![payload.id.clone()], set: vec![] },
        None,
        None,
    )
}
