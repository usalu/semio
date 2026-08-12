//! 🔺️ Sparse diff construction for the `replace-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::mutation::ReplaceDocument;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramArtifactsDelta, ProgramArtifactsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceDocument, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.artifacts.iter().find(|row| row.header.id == payload.document.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.document).expect("diff_patch always produces a full patch");
    ProgramDiff { documents: Some(ProgramArtifactsDelta { patched: vec![ProgramArtifactsPatchEntry { id: payload.document.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
