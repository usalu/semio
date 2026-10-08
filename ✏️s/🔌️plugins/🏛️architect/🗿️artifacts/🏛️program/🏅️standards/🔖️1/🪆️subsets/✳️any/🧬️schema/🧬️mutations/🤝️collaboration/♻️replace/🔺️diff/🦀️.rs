//! 🔺️ Sparse diff construction for the `replace-collaboration-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🤝collaboration` per Wave C.

use super::ReplaceCollaborationRecord;
use crate::diff::ProgramCollaborationDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceCollaborationRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.collaboration_record.header.id;
    let Some(position) = base.collaboration.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No collaboration record exists with this id.", [id.0.clone()]);
    };
    if base.collaboration[position] == payload.collaboration_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This collaboration record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramCollaborationDelta::removal(&base.collaboration, position);
    delta.absorb(ProgramCollaborationDelta::insertion(position, payload.collaboration_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { collaboration: Some(delta), ..Default::default() })
}
