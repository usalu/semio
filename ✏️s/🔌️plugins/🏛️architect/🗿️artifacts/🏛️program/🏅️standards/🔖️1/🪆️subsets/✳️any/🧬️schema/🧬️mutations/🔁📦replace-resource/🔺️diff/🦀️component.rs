//! 🔺️ Sparse diff construction for the `replace-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::mutation::ReplaceResource;
use crate::artifacts::program::diff::{ProgramResourcesDelta, ProgramResourcesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceResource, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.resources.iter().find(|row| row.header.id == payload.resource.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.resource).expect("diff_patch always produces a full patch");
    ProgramDiff { resources: Some(ProgramResourcesDelta { patched: vec![ProgramResourcesPatchEntry { id: payload.resource.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
