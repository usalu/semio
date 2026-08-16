//! 🔺️ Sparse diff construction for the `rename-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::mutation::RenameCollaborationRecord;
use crate::artifacts::program::diff::{ProgramCollaborationDelta, ProgramCollaborationPatchEntry};
use crate::artifacts::program::registers::CollaborationRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameCollaborationRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = CollaborationRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { collaboration: Some(ProgramCollaborationDelta { patched: vec![ProgramCollaborationPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
