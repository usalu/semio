//! 🔺️ Sparse diff construction for the `create-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::mutation::CreateConstraintRecord;
use crate::artifacts::program::diff::ProgramConstraintsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateConstraintRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.constraint_record.header.id.clone();
    if base.constraints.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A constraint record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { constraints: Some(ProgramConstraintsDelta { added: vec![payload.constraint_record.clone()], ..Default::default() }), ..Default::default() })
}
