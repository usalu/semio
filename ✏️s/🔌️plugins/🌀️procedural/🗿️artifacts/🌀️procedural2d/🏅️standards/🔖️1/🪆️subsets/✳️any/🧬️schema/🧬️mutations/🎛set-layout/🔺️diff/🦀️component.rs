//! 🔺️ Sparse diff builder for `MoveWidget` — a real id-keyed upsert into the fixture's layout
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn diff(payload: &super::mutation::MoveWidget, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff { removed: vec![], set: vec![(payload.id.clone(), payload.layout.clone())] }, None, None)
}
