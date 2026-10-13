//! 🔺️ Sparse diff construction for the `delete-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::DeleteDecision;
use crate::standards::v1::subsets::any::schema::diff::ProgramDecisionsDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteDecision, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.decisions.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No decision exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { decisions: Some(ProgramDecisionsDelta::removal(&base.decisions, position)), ..Default::default() })
}
