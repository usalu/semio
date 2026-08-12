//! 🔺️ Sparse diff builder for `DeleteWidget` — a real id-keyed removal from the fixture's widget
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn diff(payload: &super::mutation::DeleteWidget, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![payload.id.clone()], set: vec![] }, SynapsesDiff::default(), LayoutDiff::default(), None, None)
}
