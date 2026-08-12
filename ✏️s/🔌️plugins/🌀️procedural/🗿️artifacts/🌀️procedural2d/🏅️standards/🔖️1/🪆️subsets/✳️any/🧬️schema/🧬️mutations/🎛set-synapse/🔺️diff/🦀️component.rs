//! 🔺️ Sparse diff builder for `ConnectSynapse` — a real id-keyed upsert into the fixture's synapse
//! collection helper (never a whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn diff(payload: &super::mutation::ConnectSynapse, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff { removed: vec![], set: vec![(payload.index, payload.synapse.clone())] }, LayoutDiff::default(), None, None)
}
