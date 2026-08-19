//! 🔺️ Sparse diff construction for the `replace-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::mutation::ReplaceDecision;
use crate::artifacts::program::diff::{ProgramDecisionsDelta, ProgramDecisionsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceDecision, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.decisions.iter().find(|row| row.header.id == payload.decision.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No decision exists with this id.", [payload.decision.header.id.0.clone()]);
    };
    if existing == &payload.decision {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This decision already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.decision).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { decisions: Some(ProgramDecisionsDelta { patched: vec![ProgramDecisionsPatchEntry { id: payload.decision.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
