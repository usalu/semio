//! 🔺️ Sparse diff construction for the `replace-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::ReplaceKnowledgeRecord;
use crate::diff::ProgramKnowledgeDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.knowledge_record.header.id;
    let Some(position) = base.knowledge_payload.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No knowledge record exists with this id.", [id.0.clone()]);
    };
    if base.knowledge_payload[position] == payload.knowledge_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This knowledge record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.knowledge_payload.len()).then(|| base.knowledge_payload.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { removed: vec![id.0.clone()], added: vec![payload.knowledge_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
