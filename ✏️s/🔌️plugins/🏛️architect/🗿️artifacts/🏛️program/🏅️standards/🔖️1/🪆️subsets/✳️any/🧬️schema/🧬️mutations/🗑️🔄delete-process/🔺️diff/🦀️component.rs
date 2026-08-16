//! 🔺️ Sparse diff construction for the `delete-process` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔄processes` per Wave C.

use super::mutation::DeleteProcess;
use crate::artifacts::program::diff::ProgramProcessesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteProcess, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { processes: Some(ProgramProcessesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
