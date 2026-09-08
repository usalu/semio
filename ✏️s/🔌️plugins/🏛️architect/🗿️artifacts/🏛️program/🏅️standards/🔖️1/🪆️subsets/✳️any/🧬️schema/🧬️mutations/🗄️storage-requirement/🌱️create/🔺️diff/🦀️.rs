//! 🔺️ Sparse diff construction for the `create-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::CreateStorageRequirement;
use crate::diff::ProgramStorageDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateStorageRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.storage_requirement.header.id.clone();
    if base.storage.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A storage requirement already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { storage: Some(ProgramStorageDelta { added: vec![payload.storage_requirement.clone()], ..Default::default() }), ..Default::default() })
}
