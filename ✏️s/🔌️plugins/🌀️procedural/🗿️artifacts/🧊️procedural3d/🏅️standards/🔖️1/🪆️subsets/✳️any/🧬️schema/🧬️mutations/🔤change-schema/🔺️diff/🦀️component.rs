//! 🔺️ `change-schema` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::change_schema::mutation::ChangeSchema;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta touching only the schema field. Whole-artifact scope —
/// there is exactly one schema field, so no missing-target case exists here.
pub async fn diff(payload: &ChangeSchema, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    if payload.new_schema.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Schema id must not be empty.", Vec::<String>::new());
    }
    if base.fixture.schema == payload.new_schema {
        return protocol::MutationOutcome::new(Procedural3dDiff::default()).warn("mutation.no-op", format!("Schema is already \"{}\".", payload.new_schema));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), None, Some(payload.new_schema.clone())))
}
