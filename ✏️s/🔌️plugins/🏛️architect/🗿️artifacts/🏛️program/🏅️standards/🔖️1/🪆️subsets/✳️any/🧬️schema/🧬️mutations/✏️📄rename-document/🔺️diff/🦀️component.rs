//! 🔺️ Sparse diff construction for the `rename-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::mutation::RenameDocument;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramArtifactsDelta, ProgramArtifactsPatchEntry};
use crate::artifacts::program::registers::ArtifactRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameDocument, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ArtifactRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { documents: Some(ProgramArtifactsDelta { patched: vec![ProgramArtifactsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
