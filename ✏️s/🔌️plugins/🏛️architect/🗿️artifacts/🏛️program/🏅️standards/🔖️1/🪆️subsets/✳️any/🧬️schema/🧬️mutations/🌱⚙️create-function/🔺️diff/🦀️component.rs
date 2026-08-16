//! 🔺️ Sparse diff construction for the `create-function` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚙️functions` per Wave C.

use super::mutation::CreateFunction;
use crate::artifacts::program::diff::ProgramFunctionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.functions` on apply.
pub fn diff(payload: &CreateFunction, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { functions: Some(ProgramFunctionsDelta { added: vec![payload.function.clone()], ..Default::default() }), ..Default::default() }
}
