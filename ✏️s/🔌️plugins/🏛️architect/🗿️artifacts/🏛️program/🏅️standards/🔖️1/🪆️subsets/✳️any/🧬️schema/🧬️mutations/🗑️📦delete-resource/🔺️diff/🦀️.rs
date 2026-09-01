//! 🔺️ Sparse diff construction for the `delete-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::DeleteResource;
use crate::artifacts::program::diff::ProgramResourcesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.resources.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resource exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(ProgramResourcesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
