//! 🔺️ Sparse diff construction for the `create-template-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📐templates` per Wave C.

use super::CreateTemplateRecord;
use crate::diff::ProgramTemplatesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateTemplateRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.template_record.header.id;
    if base.templates.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A template record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.templates.len());
    if at > base.templates.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the template record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { templates: Some(ProgramTemplatesDelta::insertion(at, payload.template_record.clone())), ..Default::default() })
}
