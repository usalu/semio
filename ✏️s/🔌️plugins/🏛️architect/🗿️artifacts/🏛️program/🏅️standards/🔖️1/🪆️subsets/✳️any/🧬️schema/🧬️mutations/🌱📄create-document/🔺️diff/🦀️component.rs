//! 🔺️ Sparse diff construction for the `create-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::mutation::CreateDocument;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramArtifactsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.artifacts` on apply.
pub fn diff(payload: &CreateDocument, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { documents: Some(ProgramArtifactsDelta { added: vec![payload.document.clone()], ..Default::default() }), ..Default::default() }
}
