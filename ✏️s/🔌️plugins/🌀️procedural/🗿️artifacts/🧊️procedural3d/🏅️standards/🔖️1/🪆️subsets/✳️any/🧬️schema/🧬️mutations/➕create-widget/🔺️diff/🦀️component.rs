//! 🔺️ `create-widget` sparse diff construction — a single `WidgetsDiff.set` entry, never a
//! snapshot clone.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::create_widget::mutation::CreateWidget;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta for one `create-widget` payload.
pub fn diff(payload: &CreateWidget, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(
        base,
        WidgetsDiff { removed: vec![], set: vec![(payload.index, payload.widget.clone())] },
        SynapsesDiff::default(),
        LayoutDiff::default(),
        None,
        None,
    )
}
