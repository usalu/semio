//! 🔺️ Sparse diff construction for the `delete-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::DeleteResource;
use crate::standards::v1::subsets::any::schema::diff::ProgramResourcesDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.resources.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resource exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(ProgramResourcesDelta::removal(&base.resources, position)), ..Default::default() })
}
