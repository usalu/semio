//! 🔺️ Sparse diff construction for the `create-information-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `ℹ️information` per Wave C.

use super::CreateInformationRequirement;
use crate::diff::ProgramInformationDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateInformationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.information_requirement.header.id;
    if base.information.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An information requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.information.len());
    if at > base.information.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the information requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { information: Some(ProgramInformationDelta::insertion(at, payload.information_requirement.clone())), ..Default::default() })
}
