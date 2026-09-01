//! 🔺️ Sparse diff construction for the `replace-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::ReplaceOperationalRequirement;
use crate::artifacts::program::diff::{ProgramOperationsDelta, ProgramOperationsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceOperationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.operations.iter().find(|row| row.header.id == payload.operational_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No operational requirement exists with this id.", [payload.operational_requirement.header.id.0.clone()]);
    };
    if existing == &payload.operational_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This operational requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.operational_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { operations: Some(ProgramOperationsDelta { patched: vec![ProgramOperationsPatchEntry { id: payload.operational_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
