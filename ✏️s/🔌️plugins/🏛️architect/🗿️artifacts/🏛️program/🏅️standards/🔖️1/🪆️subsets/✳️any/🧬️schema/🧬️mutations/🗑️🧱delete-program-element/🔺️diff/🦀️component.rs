//! 🔺️ Sparse diff construction for the `delete-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::mutation::DeleteProgramElement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramElementsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteProgramElement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { elements: Some(ProgramElementsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
