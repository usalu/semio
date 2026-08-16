//! 🔺️ Sparse diff construction for the `create-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::mutation::CreateDecision;
use crate::artifacts::program::diff::ProgramDecisionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateDecision, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.decision.header.id.clone();
    if base.decisions.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A decision already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { decisions: Some(ProgramDecisionsDelta { added: vec![payload.decision.clone()], ..Default::default() }), ..Default::default() })
}
