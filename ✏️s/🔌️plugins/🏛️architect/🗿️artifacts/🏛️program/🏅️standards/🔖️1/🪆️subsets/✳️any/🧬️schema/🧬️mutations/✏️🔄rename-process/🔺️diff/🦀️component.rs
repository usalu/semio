//! 🔺️ Sparse diff construction for the `rename-process` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔄processes` per Wave C.

use super::mutation::RenameProcess;
use crate::artifacts::program::diff::{ProgramProcessesDelta, ProgramProcessesPatchEntry};
use crate::artifacts::program::registers::ProcessPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameProcess, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ProcessPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { processes: Some(ProgramProcessesDelta { patched: vec![ProgramProcessesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
