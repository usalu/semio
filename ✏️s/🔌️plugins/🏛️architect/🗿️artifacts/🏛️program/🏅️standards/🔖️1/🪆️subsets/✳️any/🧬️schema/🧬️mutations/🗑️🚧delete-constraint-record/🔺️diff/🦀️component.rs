//! 🔺️ Sparse diff construction for the `delete-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::mutation::DeleteConstraintRecord;
use crate::artifacts::program::diff::ProgramConstraintsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteConstraintRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.constraints.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No constraint record exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { constraints: Some(ProgramConstraintsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
