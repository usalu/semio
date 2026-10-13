//! 🔺️ Sparse diff construction for the `delete-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::DeleteConstraintRecord;
use crate::standards::v1::subsets::any::schema::diff::ProgramConstraintsDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteConstraintRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.constraints.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No constraint record exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { constraints: Some(ProgramConstraintsDelta::removal(&base.constraints, position)), ..Default::default() })
}
