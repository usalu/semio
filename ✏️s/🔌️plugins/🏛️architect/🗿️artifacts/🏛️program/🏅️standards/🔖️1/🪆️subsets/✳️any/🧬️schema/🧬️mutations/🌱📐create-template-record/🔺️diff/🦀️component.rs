//! 🔺️ Sparse diff construction for the `create-template-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📐templates` per Wave C.

use super::mutation::CreateTemplateRecord;
use crate::artifacts::program::diff::ProgramTemplatesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.templates` on apply.
pub fn diff(payload: &CreateTemplateRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { templates: Some(ProgramTemplatesDelta { added: vec![payload.template_record.clone()], ..Default::default() }), ..Default::default() }
}
