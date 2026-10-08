//! 🔺️ Sparse diff construction for the `replace-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::ReplaceKnowledgeRecord;
use crate::diff::ProgramKnowledgeDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.knowledge_record.header.id;
    let Some(position) = base.knowledge_payload.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No knowledge record exists with this id.", [id.0.clone()]);
    };
    if base.knowledge_payload[position] == payload.knowledge_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This knowledge record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramKnowledgeDelta::removal(&base.knowledge_payload, position);
    delta.absorb(ProgramKnowledgeDelta::insertion(position, payload.knowledge_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(delta), ..Default::default() })
}
