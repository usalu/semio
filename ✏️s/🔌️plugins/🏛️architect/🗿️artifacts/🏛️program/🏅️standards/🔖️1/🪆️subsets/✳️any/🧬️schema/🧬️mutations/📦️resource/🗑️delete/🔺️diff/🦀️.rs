//! 🔺️ Sparse diff construction for the `delete-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::DeleteResource;
use crate::diff::ProgramResourcesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.resources.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resource exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(ProgramResourcesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
