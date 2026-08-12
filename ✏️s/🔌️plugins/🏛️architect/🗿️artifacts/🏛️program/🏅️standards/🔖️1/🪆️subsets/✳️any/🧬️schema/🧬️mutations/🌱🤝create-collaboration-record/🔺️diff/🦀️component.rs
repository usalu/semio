//! 🔺️ Sparse diff construction for the `create-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::mutation::CreateCollaborationRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramCollaborationDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.collaboration` on apply.
pub fn diff(payload: &CreateCollaborationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { collaboration: Some(ProgramCollaborationDelta { added: vec![payload.collaboration_record.clone()], ..Default::default() }), ..Default::default() }
}
