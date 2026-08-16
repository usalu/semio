//! 🔺️ Sparse diff construction for the `delete-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::mutation::DeleteCollaborationRecord;
use crate::artifacts::program::diff::ProgramCollaborationDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteCollaborationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { collaboration: Some(ProgramCollaborationDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
