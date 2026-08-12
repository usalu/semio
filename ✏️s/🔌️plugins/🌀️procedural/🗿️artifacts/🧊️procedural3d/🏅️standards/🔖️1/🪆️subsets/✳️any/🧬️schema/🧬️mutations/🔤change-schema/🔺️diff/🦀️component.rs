//! 🔺️ `change-schema` sparse diff construction.

use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::change_schema::mutation::ChangeSchema;
use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// 🏗️ Builds the sparse fixture delta touching only the schema field.
pub fn diff(payload: &ChangeSchema, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    diff_fixture_from_helpers(base, WidgetsDiff::default(), SynapsesDiff::default(), LayoutDiff::default(), None, Some(payload.new_schema.clone()))
}
