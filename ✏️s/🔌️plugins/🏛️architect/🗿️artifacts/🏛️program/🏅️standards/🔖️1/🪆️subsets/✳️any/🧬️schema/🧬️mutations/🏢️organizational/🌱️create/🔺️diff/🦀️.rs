//! 🔺️ Sparse diff construction for the `create-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::CreateOrganizationalRequirement;
use crate::diff::ProgramOrganizationalDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateOrganizationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.organizational_requirement.header.id;
    if base.organizational.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An organizational requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.organizational.len());
    if at > base.organizational.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the organizational requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { organizational: Some(ProgramOrganizationalDelta::insertion(at, payload.organizational_requirement.clone())), ..Default::default() })
}
