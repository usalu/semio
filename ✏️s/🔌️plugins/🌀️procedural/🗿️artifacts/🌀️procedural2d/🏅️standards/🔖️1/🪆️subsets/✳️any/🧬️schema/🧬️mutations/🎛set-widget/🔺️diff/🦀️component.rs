//! 🔺️ Sparse diff builder for `CreateWidget` — a real id-keyed upsert into the fixture's widget
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn diff(payload: &super::mutation::CreateWidget, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![], set: vec![(payload.index, payload.widget.clone())] }, SynapsesDiff::default(), LayoutDiff::default(), None, None)
}
