//! 🔺️ Sparse diff construction for the `create-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::mutation::CreateOperationalRequirement;
use crate::artifacts::program::diff::ProgramOperationsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateOperationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.operational_requirement.header.id.clone();
    if base.operations.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An operational requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { operations: Some(ProgramOperationsDelta { added: vec![payload.operational_requirement.clone()], ..Default::default() }), ..Default::default() })
}
