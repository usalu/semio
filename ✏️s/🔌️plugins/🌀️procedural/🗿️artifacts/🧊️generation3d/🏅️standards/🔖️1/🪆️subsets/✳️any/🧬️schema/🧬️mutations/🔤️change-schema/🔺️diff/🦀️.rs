//! 🔺️ `change-schema` sparse diff construction.

use crate::diff::Generation3dDiff;
use crate::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::mutations::change_schema::ChangeSchema;
use crate::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta touching only the schema field. Whole-artifact scope —
/// there is exactly one schema field, so no missing-target case exists here.
pub fn diff(payload: &ChangeSchema, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if payload.new_schema.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Schema id must not be empty.", Vec::<String>::new());
    }
    if base.fixture.schema == payload.new_schema {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warn("mutation.no-op", format!("Schema is already \"{}\".", payload.new_schema));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), None, Some(payload.new_schema.clone())))
}
