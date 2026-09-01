//! 🔺️ Sparse diff construction for the `delete-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::DeleteDecision;
use crate::artifacts::program::diff::ProgramDecisionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteDecision, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.decisions.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No decision exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { decisions: Some(ProgramDecisionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
