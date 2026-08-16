//! 🔺️ Sparse diff construction for the `rename-template-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📐templates` per Wave C.

use super::mutation::RenameTemplateRecord;
use crate::artifacts::program::diff::{ProgramTemplatesDelta, ProgramTemplatesPatchEntry};
use crate::artifacts::program::registers::TemplateRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameTemplateRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = TemplateRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { templates: Some(ProgramTemplatesDelta { patched: vec![ProgramTemplatesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
