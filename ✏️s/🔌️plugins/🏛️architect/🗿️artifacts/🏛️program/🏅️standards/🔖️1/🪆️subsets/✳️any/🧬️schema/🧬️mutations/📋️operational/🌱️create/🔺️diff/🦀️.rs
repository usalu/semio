//! 🔺️ Sparse diff construction for the `create-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::CreateOperationalRequirement;
use crate::diff::ProgramOperationsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateOperationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.operational_requirement.header.id;
    if base.operations.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An operational requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.operations.len());
    if at > base.operations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the operational requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { operations: Some(ProgramOperationsDelta::insertion(at, payload.operational_requirement.clone())), ..Default::default() })
}
