//! 🔺️ Sparse diff builder for `MoveWidget` — a real id-keyed upsert into the fixture's layout
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};

pub fn diff(payload: &super::mutation::MoveWidget, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    if !base.fixture.widgets.iter().any(|widget| widget_id(widget) == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !payload.layout.x.is_finite() || !payload.layout.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Widget \"{}\" position must be finite.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff { removed: vec![], set: vec![(payload.id.clone(), payload.layout.clone())] }, None, None))
}
