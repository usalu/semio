//! 🔺️ Sparse diff construction for the `create-process` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔄processes` per Wave C.

use super::mutation::CreateProcess;
use crate::artifacts::program::diff::ProgramProcessesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.processes` on apply.
pub fn diff(payload: &CreateProcess, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { processes: Some(ProgramProcessesDelta { added: vec![payload.process.clone()], ..Default::default() }), ..Default::default() }
}
