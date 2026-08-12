//! 🔺️ `delete-widget` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::delete_widget::mutation::DeleteWidget;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta removing one widget by id.
pub fn diff(payload: &DeleteWidget, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(
        base,
        WidgetsDiff { removed: vec![payload.id.clone()], set: vec![] },
        SynapsesDiff::default(),
        LayoutDiff::default(),
        None,
        None,
    )
}
