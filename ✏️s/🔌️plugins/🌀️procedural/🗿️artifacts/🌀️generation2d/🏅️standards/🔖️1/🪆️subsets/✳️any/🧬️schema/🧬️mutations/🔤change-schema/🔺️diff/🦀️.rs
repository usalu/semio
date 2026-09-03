//! 🔺️ Sparse diff builder for `ChangeSchema` — a real scalar-field write on the fixture (never a
//! whole-snapshot capture).

use crate::artifacts::generation2d::diff::{diff_fixture_from_helpers, LayoutDiff, Generation2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::generation2d::Generation2dSnapshot;

pub fn diff(payload: &super::ChangeSchema, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
    if base.fixture.schema == payload.schema {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fixture schema is already \"{}\".", payload.schema));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), None, Some(payload.schema.clone())))
}
