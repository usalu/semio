//! 🔺️ Sparse diff construction for the `replace-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::ReplaceConstraintRecord;
use crate::artifacts::program::diff::{ProgramConstraintsDelta, ProgramConstraintsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceConstraintRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.constraints.iter().find(|row| row.header.id == payload.constraint_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No constraint record exists with this id.", [payload.constraint_record.header.id.0.clone()]);
    };
    if existing == &payload.constraint_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This constraint record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.constraint_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { constraints: Some(ProgramConstraintsDelta { patched: vec![ProgramConstraintsPatchEntry { id: payload.constraint_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
