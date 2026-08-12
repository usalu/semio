//! 🔺️ Sparse diff construction for the `delete-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::mutation::DeleteDocument;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramArtifactsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteDocument, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { documents: Some(ProgramArtifactsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
