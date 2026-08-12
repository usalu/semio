//! 🔺️ Sparse diff construction for the `delete-change-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📝changes` per Wave C.

use super::mutation::DeleteChangeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramChangesDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteChangeRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { changes: Some(ProgramChangesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
