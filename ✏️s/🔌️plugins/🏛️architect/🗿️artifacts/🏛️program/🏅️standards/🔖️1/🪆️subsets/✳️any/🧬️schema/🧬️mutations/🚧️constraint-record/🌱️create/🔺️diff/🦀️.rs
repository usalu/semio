//! 🔺️ Sparse diff construction for the `create-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::CreateConstraintRecord;
use crate::diff::ProgramConstraintsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateConstraintRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.constraint_record.header.id;
    if base.constraints.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A constraint record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.constraints.len());
    if at > base.constraints.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the constraint record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { constraints: Some(ProgramConstraintsDelta::insertion(at, payload.constraint_record.clone())), ..Default::default() })
}
