//! 🔺️ Sparse diff builder for `ChangeSchema` — a real scalar-field write on the fixture (never a
//! whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn diff(payload: &super::mutation::ChangeSchema, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), None, Some(payload.schema.clone()))
}
