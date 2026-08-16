//! 🔺️ Sparse diff construction for the `delete-function` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚙️functions` per Wave C.

use super::mutation::DeleteFunction;
use crate::artifacts::program::diff::ProgramFunctionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteFunction, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { functions: Some(ProgramFunctionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
