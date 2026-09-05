//! 🔺️ `create-widget` sparse diff construction — a single `WidgetsDiff.set` entry, never a
//! snapshot clone.

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::generation3d::mutations::create_widget::CreateWidget;
use crate::artifacts::generation3d::mutations::widget_index;
use crate::artifacts::generation3d::{widget_id, Generation3dSnapshot};

/// 🏗️ Builds the sparse fixture delta for one `create-widget` payload.
pub fn diff(payload: &CreateWidget, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = widget_id(&payload.widget);
    if widget_index(&base.fixture, id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{id}\" already exists."), [id.to_string()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![], set: vec![(payload.index, payload.widget.clone())] }, SynapsesDiff::default(), LayoutDiff::default(), None, None))
}
