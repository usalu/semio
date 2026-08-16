//! 🔺️ Sparse diff construction for the `create-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::mutation::CreateStorageRequirement;
use crate::artifacts::program::diff::ProgramStorageDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.storage` on apply.
pub fn diff(payload: &CreateStorageRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { storage: Some(ProgramStorageDelta { added: vec![payload.storage_requirement.clone()], ..Default::default() }), ..Default::default() }
}
