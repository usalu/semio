//! 🔺️ Sparse diff builder for `ClearWidgetLayout` — a real id-keyed removal from the fixture's
//! layout collection helper (never a whole-snapshot capture).

use crate::artifacts::generation2d::diff::{diff_fixture_from_helpers, LayoutDiff, Generation2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::generation2d::Generation2dSnapshot;

pub fn diff(payload: &super::ClearWidgetLayout, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if !base.fixture.layout.contains_key(&payload.id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Widget \"{}\" already has no layout entry.", payload.id));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff { removed: vec![payload.id.clone()], set: vec![] }, None, None))
}
