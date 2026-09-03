//! 🔺️ Sparse diff builder for `CreateWidget` — a real id-keyed upsert into the fixture's widget
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::generation2d::diff::{diff_fixture_from_helpers, LayoutDiff, Generation2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::generation2d::{widget_id, Generation2dSnapshot};

pub fn diff(payload: &super::CreateWidget, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    let id = widget_id(&payload.widget);
    if base.fixture.widgets.iter().any(|widget| widget_id(widget) == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{id}\" already exists."), [id.to_string()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![], set: vec![(payload.index, payload.widget.clone())] }, SynapsesDiff::default(), LayoutDiff::default(), None, None))
}
