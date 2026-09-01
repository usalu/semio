//! 🔺️ Sparse diff construction for the `delete-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::DeleteRisk;
use crate::artifacts::program::diff::ProgramRisksDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteRisk, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.risks.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No risk exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { risks: Some(ProgramRisksDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
