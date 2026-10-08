//! 🔺️ Sparse diff construction for the `create-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::CreateStorageRequirement;
use crate::diff::ProgramStorageDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateStorageRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.storage_requirement.header.id;
    if base.storage.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A storage requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.storage.len());
    if at > base.storage.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the storage requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { storage: Some(ProgramStorageDelta::insertion(at, payload.storage_requirement.clone())), ..Default::default() })
}
