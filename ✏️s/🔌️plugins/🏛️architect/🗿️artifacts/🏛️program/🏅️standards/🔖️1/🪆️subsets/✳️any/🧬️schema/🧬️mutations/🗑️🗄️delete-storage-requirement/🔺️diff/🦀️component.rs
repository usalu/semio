//! 🔺️ Sparse diff construction for the `delete-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::mutation::DeleteStorageRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramStorageDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteStorageRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { storage: Some(ProgramStorageDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
