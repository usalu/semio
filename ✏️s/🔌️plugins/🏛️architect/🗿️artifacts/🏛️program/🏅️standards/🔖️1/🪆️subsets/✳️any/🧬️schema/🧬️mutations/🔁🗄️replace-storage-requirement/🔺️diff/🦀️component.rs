//! 🔺️ Sparse diff construction for the `replace-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::mutation::ReplaceStorageRequirement;
use crate::artifacts::program::diff::{ProgramStorageDelta, ProgramStoragePatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub fn diff(payload: &ReplaceStorageRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.storage.iter().find(|row| row.header.id == payload.storage_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No storage requirement exists with this id.", [payload.storage_requirement.header.id.0.clone()]);
    };
    if existing == &payload.storage_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This storage requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.storage_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { storage: Some(ProgramStorageDelta { patched: vec![ProgramStoragePatchEntry { id: payload.storage_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
