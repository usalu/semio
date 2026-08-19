//! 🔺️ Sparse diff construction for the `replace-change-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📝changes` per Wave C.

use super::mutation::ReplaceChangeRecord;
use crate::artifacts::program::diff::{ProgramChangesDelta, ProgramChangesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceChangeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.changes.iter().find(|row| row.header.id == payload.change_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No change record exists with this id.", [payload.change_record.header.id.0.clone()]);
    };
    if existing == &payload.change_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This change record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.change_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { changes: Some(ProgramChangesDelta { patched: vec![ProgramChangesPatchEntry { id: payload.change_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
