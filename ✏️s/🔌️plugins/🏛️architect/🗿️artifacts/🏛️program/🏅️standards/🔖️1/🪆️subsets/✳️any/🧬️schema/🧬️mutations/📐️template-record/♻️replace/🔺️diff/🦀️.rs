//! 🔺️ Sparse diff construction for the `replace-template-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📐templates` per Wave C.

use super::ReplaceTemplateRecord;
use crate::artifacts::program::diff::{ProgramTemplatesDelta, ProgramTemplatesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceTemplateRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.templates.iter().find(|row| row.header.id == payload.template_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No template record exists with this id.", [payload.template_record.header.id.0.clone()]);
    };
    if existing == &payload.template_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This template record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.template_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { templates: Some(ProgramTemplatesDelta { patched: vec![ProgramTemplatesPatchEntry { id: payload.template_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
