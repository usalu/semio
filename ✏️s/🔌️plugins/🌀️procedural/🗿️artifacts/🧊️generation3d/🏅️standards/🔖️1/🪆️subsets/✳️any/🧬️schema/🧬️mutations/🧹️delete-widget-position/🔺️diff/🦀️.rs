//! 🔺️ `delete-widget-position` sparse diff construction.

use crate::diff::Generation3dDiff;
use crate::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::mutations::delete_widget_position::DeleteWidgetPosition;
use crate::mutations::widget_index;
use crate::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta removing one widget's position override.
pub fn diff(payload: &DeleteWidgetPosition, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if widget_index(&base.fixture, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !base.fixture.layout.contains_key(&payload.id) {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warn("mutation.no-op", format!("Widget \"{}\" already has no position override.", payload.id));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff { removed: vec![payload.id.clone()], set: vec![] }, None, None))
}
