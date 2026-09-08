//! 🔺️ `delete-widget` sparse diff construction.

use crate::diff::Generation3dDiff;
use crate::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::mutations::delete_widget::DeleteWidget;
use crate::mutations::widget_index;
use crate::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta removing one widget by id.
pub fn diff(payload: &DeleteWidget, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if widget_index(&base.fixture, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![payload.id.clone()], set: vec![] }, SynapsesDiff::default(), LayoutDiff::default(), None, None))
}
