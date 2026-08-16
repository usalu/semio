//! 🔺️ Sparse diff construction for the `delete-template-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📐templates` per Wave C.

use super::mutation::DeleteTemplateRecord;
use crate::artifacts::program::diff::ProgramTemplatesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteTemplateRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { templates: Some(ProgramTemplatesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
