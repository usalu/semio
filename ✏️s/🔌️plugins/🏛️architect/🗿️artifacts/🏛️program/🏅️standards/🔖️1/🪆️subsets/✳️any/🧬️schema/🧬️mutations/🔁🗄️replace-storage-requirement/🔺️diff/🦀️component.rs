//! 🔺️ Sparse diff construction for the `replace-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::mutation::ReplaceStorageRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramStorageDelta, ProgramStoragePatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceStorageRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.storage.iter().find(|row| row.header.id == payload.storage_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.storage_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { storage: Some(ProgramStorageDelta { patched: vec![ProgramStoragePatchEntry { id: payload.storage_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
