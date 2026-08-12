//! 🔺️ Sparse diff construction for the `functions` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateFunction, DeleteFunction, RenameFunction, ReplaceFunction};
use crate::artifacts::program::diff::{ProgramFunctionsDelta, ProgramFunctionsPatchEntry};
use crate::artifacts::program::registers::FunctionPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.functions` on apply.
pub fn diff_create(payload: &CreateFunction, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { functions: Some(ProgramFunctionsDelta { added: vec![payload.function.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteFunction, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { functions: Some(ProgramFunctionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameFunction, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = FunctionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { functions: Some(ProgramFunctionsDelta { patched: vec![ProgramFunctionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceFunction, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.functions.iter().find(|row| row.header.id == payload.function.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.function).expect("diff_patch always produces a full patch");
    ProgramDiff { functions: Some(ProgramFunctionsDelta { patched: vec![ProgramFunctionsPatchEntry { id: payload.function.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
