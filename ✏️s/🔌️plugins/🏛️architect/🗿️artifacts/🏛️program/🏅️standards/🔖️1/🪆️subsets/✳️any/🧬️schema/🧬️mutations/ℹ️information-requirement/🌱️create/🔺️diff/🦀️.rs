//! 🔺️ Sparse diff construction for the `create-information-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `ℹ️information` per Wave C.

use super::CreateInformationRequirement;
use crate::diff::ProgramInformationDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateInformationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.information_requirement.header.id.clone();
    if base.information.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An information requirement already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { information: Some(ProgramInformationDelta { added: vec![payload.information_requirement.clone()], ..Default::default() }), ..Default::default() })
}
