//! 🔺️ `delete-widget-position` sparse diff construction.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::delete_widget_position::DeleteWidgetPosition;
use crate::artifacts::procedural3d::mutations::widget_index;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta removing one widget's position override.
pub fn diff(payload: &DeleteWidgetPosition, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    if widget_index(&base.fixture, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !base.fixture.layout.contains_key(&payload.id) {
        return protocol::MutationOutcome::new(Procedural3dDiff::default()).warn("mutation.no-op", format!("Widget \"{}\" already has no position override.", payload.id));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff { removed: vec![payload.id.clone()], set: vec![] }, None, None))
}
