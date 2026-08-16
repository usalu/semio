//! 🔺️ Sparse diff construction for the `rename-function` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚙️functions` per Wave C.

use super::mutation::RenameFunction;
use crate::artifacts::program::diff::{ProgramFunctionsDelta, ProgramFunctionsPatchEntry};
use crate::artifacts::program::registers::FunctionPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameFunction, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = FunctionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { functions: Some(ProgramFunctionsDelta { patched: vec![ProgramFunctionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
