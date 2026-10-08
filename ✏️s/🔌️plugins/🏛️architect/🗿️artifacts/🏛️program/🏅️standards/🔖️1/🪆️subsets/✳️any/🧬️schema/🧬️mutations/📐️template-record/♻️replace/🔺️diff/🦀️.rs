//! 🔺️ Sparse diff construction for the `replace-template-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📐templates` per Wave C.

use super::ReplaceTemplateRecord;
use crate::diff::ProgramTemplatesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceTemplateRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.template_record.header.id;
    let Some(position) = base.templates.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No template record exists with this id.", [id.0.clone()]);
    };
    if base.templates[position] == payload.template_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This template record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramTemplatesDelta::removal(&base.templates, position);
    delta.absorb(ProgramTemplatesDelta::insertion(position, payload.template_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { templates: Some(delta), ..Default::default() })
}
