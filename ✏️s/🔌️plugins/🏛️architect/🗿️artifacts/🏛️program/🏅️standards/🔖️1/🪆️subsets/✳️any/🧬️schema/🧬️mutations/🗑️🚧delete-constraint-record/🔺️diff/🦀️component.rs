//! 🔺️ Sparse diff construction for the `delete-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::mutation::DeleteConstraintRecord;
use crate::artifacts::program::diff::ProgramConstraintsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteConstraintRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { constraints: Some(ProgramConstraintsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
