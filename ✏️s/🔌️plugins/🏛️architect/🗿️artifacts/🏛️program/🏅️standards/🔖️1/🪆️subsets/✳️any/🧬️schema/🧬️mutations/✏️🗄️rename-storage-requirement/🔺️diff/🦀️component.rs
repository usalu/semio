//! 🔺️ Sparse diff construction for the `rename-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::mutation::RenameStorageRequirement;
use crate::artifacts::program::diff::{ProgramStorageDelta, ProgramStoragePatchEntry};
use crate::artifacts::program::registers::StorageRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameStorageRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = StorageRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { storage: Some(ProgramStorageDelta { patched: vec![ProgramStoragePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
