//! 🔺️ `change-schema` sparse diff construction.

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;

use crate::change_schema::ChangeSchema;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// 🏗️ Builds the sparse fixture delta touching only the schema field. Whole-artifact scope —
/// there is exactly one schema field, so no missing-target case exists here.
pub fn diff(payload: &ChangeSchema, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if payload.new_schema.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Schema id must not be empty.", Vec::<String>::new());
    }
    if base.host_snapshot.schema == payload.new_schema {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", format!("Schema is already \"{}\".", payload.new_schema));
    }
    protocol::MutationOutcome::new(Generation3dDiff { schema: Some(payload.new_schema.clone()), ..Default::default() })
}
