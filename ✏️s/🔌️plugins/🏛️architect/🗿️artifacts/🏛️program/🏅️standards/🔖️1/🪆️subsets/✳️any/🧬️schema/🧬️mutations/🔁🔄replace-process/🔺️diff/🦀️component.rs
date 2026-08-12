//! 🔺️ Sparse diff construction for the `replace-process` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔄processes` per Wave C.

use super::mutation::ReplaceProcess;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramProcessesDelta, ProgramProcessesPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceProcess, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.processes.iter().find(|row| row.header.id == payload.process.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.process).expect("diff_patch always produces a full patch");
    ProgramDiff { processes: Some(ProgramProcessesDelta { patched: vec![ProgramProcessesPatchEntry { id: payload.process.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
