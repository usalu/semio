//! 🔺️ Sparse diff construction for the `replace-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::ReplaceDecision;
use crate::diff::ProgramDecisionsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceDecision, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.decision.header.id;
    let Some(position) = base.decisions.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No decision exists with this id.", [id.0.clone()]);
    };
    if base.decisions[position] == payload.decision {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This decision already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramDecisionsDelta::removal(&base.decisions, position);
    delta.absorb(ProgramDecisionsDelta::insertion(position, payload.decision.clone()));
    protocol::MutationOutcome::new(ProgramDiff { decisions: Some(delta), ..Default::default() })
}
