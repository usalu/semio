//! 🔺️ Sparse diff construction for the `delete-template-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📐templates` per Wave C.

use super::mutation::DeleteTemplateRecord;
use crate::artifacts::program::diff::ProgramTemplatesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteTemplateRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.templates.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No template record exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { templates: Some(ProgramTemplatesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
