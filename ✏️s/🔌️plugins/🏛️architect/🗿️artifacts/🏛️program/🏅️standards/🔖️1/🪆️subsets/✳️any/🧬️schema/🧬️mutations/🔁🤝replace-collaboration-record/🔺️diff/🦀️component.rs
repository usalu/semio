//! 🔺️ Sparse diff construction for the `replace-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::mutation::ReplaceCollaborationRecord;
use crate::artifacts::program::diff::{ProgramCollaborationDelta, ProgramCollaborationPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceCollaborationRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.collaboration.iter().find(|row| row.header.id == payload.collaboration_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.collaboration_record).expect("diff_patch always produces a full patch");
    ProgramDiff { collaboration: Some(ProgramCollaborationDelta { patched: vec![ProgramCollaborationPatchEntry { id: payload.collaboration_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
