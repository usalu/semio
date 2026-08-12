//! 🔺️ Sparse diff construction for the `replace-communication-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📡communication` per Wave C.

use super::mutation::ReplaceCommunicationRequirement;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramCommunicationDelta, ProgramCommunicationPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceCommunicationRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.communication.iter().find(|row| row.header.id == payload.communication_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.communication_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { communication: Some(ProgramCommunicationDelta { patched: vec![ProgramCommunicationPatchEntry { id: payload.communication_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
