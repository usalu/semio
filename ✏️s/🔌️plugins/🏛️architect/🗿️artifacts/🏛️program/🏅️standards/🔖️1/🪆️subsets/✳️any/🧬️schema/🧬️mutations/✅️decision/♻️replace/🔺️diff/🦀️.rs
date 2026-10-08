//! 🔺️ Sparse diff construction for the `replace-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::ReplaceDecision;
use crate::diff::ProgramDecisionsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceDecision, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.decision.header.id;
    let Some(position) = base.decisions.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No decision exists with this id.", [id.0.clone()]);
    };
    if base.decisions[position] == payload.decision {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This decision already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.decisions.len()).then(|| base.decisions.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { decisions: Some(ProgramDecisionsDelta { removed: vec![id.0.clone()], added: vec![payload.decision.clone()], reordered, ..Default::default() }), ..Default::default() })
}
