//! 🔺️ Sparse diff construction for the `storage` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateStorageRequirement, DeleteStorageRequirement, RenameStorageRequirement, ReplaceStorageRequirement};
use crate::artifacts::program::diff::{ProgramStorageDelta, ProgramStoragePatchEntry};
use crate::artifacts::program::registers::StorageRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.storage` on apply.
pub fn diff_create(payload: &CreateStorageRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { storage: Some(ProgramStorageDelta { added: vec![payload.storage_requirement.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteStorageRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { storage: Some(ProgramStorageDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameStorageRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = StorageRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { storage: Some(ProgramStorageDelta { patched: vec![ProgramStoragePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceStorageRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.storage.iter().find(|row| row.header.id == payload.storage_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.storage_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { storage: Some(ProgramStorageDelta { patched: vec![ProgramStoragePatchEntry { id: payload.storage_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
