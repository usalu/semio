//! 🔺️ Sparse diff construction for the `rename-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::mutation::RenameStorageRequirement;
use crate::artifacts::program::diff::{ProgramStorageDelta, ProgramStoragePatchEntry};
use crate::artifacts::program::registers::StorageRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameStorageRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.storage.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No storage requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This storage requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = StorageRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { storage: Some(ProgramStorageDelta { patched: vec![ProgramStoragePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
