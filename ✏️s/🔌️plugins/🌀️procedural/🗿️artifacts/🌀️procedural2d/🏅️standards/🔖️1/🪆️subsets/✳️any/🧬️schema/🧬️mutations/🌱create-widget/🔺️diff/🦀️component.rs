//! 🔺️ Sparse diff builder for `CreateWidget` — a real id-keyed upsert into the fixture's widget
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};

pub async fn diff(payload: &super::mutation::CreateWidget, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    let id = widget_id(&payload.widget);
    if base.fixture.widgets.iter().any(|widget| widget_id(widget) == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A widget with id \"{id}\" already exists."), [id.to_string()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![], set: vec![(payload.index, payload.widget.clone())] }, SynapsesDiff::default(), LayoutDiff::default(), None, None))
}
