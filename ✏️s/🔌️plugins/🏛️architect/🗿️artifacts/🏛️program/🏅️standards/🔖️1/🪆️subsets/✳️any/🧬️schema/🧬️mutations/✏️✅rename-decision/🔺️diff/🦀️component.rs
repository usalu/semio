//! 🔺️ Sparse diff construction for the `rename-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::mutation::RenameDecision;
use crate::artifacts::program::diff::{ProgramDecisionsDelta, ProgramDecisionsPatchEntry};
use crate::artifacts::program::registers::DecisionPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameDecision, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.decisions.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No decision exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This decision already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = DecisionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { decisions: Some(ProgramDecisionsDelta { patched: vec![ProgramDecisionsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
