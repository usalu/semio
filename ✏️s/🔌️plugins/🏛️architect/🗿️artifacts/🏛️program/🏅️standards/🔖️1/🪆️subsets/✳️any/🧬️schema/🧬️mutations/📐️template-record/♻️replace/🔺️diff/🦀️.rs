//! 🔺️ Sparse diff construction for the `replace-template-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📐templates` per Wave C.

use super::ReplaceTemplateRecord;
use crate::diff::ProgramTemplatesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceTemplateRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.template_record.header.id;
    let Some(position) = base.templates.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No template record exists with this id.", [id.0.clone()]);
    };
    if base.templates[position] == payload.template_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This template record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.templates.len()).then(|| base.templates.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { templates: Some(ProgramTemplatesDelta { removed: vec![id.0.clone()], added: vec![payload.template_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
