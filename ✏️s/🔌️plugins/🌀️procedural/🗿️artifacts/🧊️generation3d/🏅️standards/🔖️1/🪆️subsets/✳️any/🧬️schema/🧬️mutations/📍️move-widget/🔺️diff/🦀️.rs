//! 🔺️ `move-widget` sparse diff construction.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::standards::v1::subsets::any::schema::mutations::move_widget::MoveWidget;
use crate::standards::v1::subsets::any::schema::mutations::widget_index;
use crate::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta upserting one widget's position.
pub fn diff(payload: &MoveWidget, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if widget_index(&base.fixture, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !payload.layout.x.is_finite() || !payload.layout.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Position for widget \"{}\" is not finite.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, &WidgetsDiff::default(), &SynapsesDiff::default(), &LayoutDiff { removed: vec![], set: vec![(payload.id.clone(), payload.layout.clone())] }, None, None))
}
