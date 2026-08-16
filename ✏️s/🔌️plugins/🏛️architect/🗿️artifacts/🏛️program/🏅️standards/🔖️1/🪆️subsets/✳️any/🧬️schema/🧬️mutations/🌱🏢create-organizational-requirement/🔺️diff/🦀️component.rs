//! 🔺️ Sparse diff construction for the `create-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::mutation::CreateOrganizationalRequirement;
use crate::artifacts::program::diff::ProgramOrganizationalDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateOrganizationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.organizational_requirement.header.id.clone();
    if base.organizational.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An organizational requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { organizational: Some(ProgramOrganizationalDelta { added: vec![payload.organizational_requirement.clone()], ..Default::default() }), ..Default::default() })
}
