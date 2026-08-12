//! 🔺️ Sparse diff construction for the `processes` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateProcess, DeleteProcess, RenameProcess, ReplaceProcess};
use crate::artifacts::program::diff::{ProgramProcessesDelta, ProgramProcessesPatchEntry};
use crate::artifacts::program::registers::ProcessPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.processes` on apply.
pub fn diff_create(payload: &CreateProcess, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { processes: Some(ProgramProcessesDelta { added: vec![payload.process.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteProcess, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { processes: Some(ProgramProcessesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameProcess, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ProcessPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { processes: Some(ProgramProcessesDelta { patched: vec![ProgramProcessesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceProcess, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.processes.iter().find(|row| row.header.id == payload.process.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.process).expect("diff_patch always produces a full patch");
    ProgramDiff { processes: Some(ProgramProcessesDelta { patched: vec![ProgramProcessesPatchEntry { id: payload.process.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
