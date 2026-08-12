//! 🔺️ Sparse diff builder for `ClearWidgetLayout` — a real id-keyed removal from the fixture's
//! layout collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn diff(payload: &super::mutation::ClearWidgetLayout, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff { removed: vec![payload.id.clone()], set: vec![] }, None, None)
}
