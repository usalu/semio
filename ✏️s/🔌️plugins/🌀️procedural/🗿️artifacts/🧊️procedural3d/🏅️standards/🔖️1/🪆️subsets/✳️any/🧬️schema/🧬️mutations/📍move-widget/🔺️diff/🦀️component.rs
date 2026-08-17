//! 🔺️ `move-widget` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::move_widget::mutation::MoveWidget;
use crate::artifacts::procedural3d::mutations::widget_index;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta upserting one widget's position.
pub fn diff(payload: &MoveWidget, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    if widget_index(&base.fixture, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !payload.layout.x.is_finite() || !payload.layout.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Position for widget \"{}\" is not finite.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(
        base,
        WidgetsDiff::default(),
        SynapsesDiff::default(),
        LayoutDiff { removed: vec![], set: vec![(payload.id.clone(), payload.layout.clone())] },
        None,
        None,
    ))
}
