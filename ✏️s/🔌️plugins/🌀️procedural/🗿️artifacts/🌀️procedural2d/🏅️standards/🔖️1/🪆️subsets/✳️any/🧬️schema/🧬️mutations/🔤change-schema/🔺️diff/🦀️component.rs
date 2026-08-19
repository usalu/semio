//! 🔺️ Sparse diff builder for `ChangeSchema` — a real scalar-field write on the fixture (never a
//! whole-snapshot capture).

use crate::artifacts::procedural2d::diff::{diff_fixture_from_helpers, LayoutDiff, Procedural2dDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub async fn diff(payload: &super::mutation::ChangeSchema, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
    if base.fixture.schema == payload.schema {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fixture schema is already \"{}\".", payload.schema));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), None, Some(payload.schema.clone())))
}
