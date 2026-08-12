//! 🔺️ `update-widget` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::update_widget::mutation::UpdateWidget;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta replacing one existing widget's body. The index is
/// irrelevant here — `apply_widgets_diff` resolves an existing entry by id before ever consulting
/// the index, which only matters for a genuinely new (`create-widget`) insertion.
pub fn diff(payload: &UpdateWidget, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(
        base,
        WidgetsDiff { removed: vec![], set: vec![(0, payload.widget.clone())] },
        SynapsesDiff::default(),
        LayoutDiff::default(),
        None,
        None,
    )
}
