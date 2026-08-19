//! 🔺️ Sparse diff construction for the `rename-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::mutation::RenameOperationalRequirement;
use crate::artifacts::program::diff::{ProgramOperationsDelta, ProgramOperationsPatchEntry};
use crate::artifacts::program::registers::OperationalRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameOperationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.operations.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No operational requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This operational requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = OperationalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { operations: Some(ProgramOperationsDelta { patched: vec![ProgramOperationsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
