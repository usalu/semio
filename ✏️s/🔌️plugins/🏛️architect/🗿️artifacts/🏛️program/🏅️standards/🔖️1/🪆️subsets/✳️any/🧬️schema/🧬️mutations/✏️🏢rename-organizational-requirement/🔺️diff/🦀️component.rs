//! 🔺️ Sparse diff construction for the `rename-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::mutation::RenameOrganizationalRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramOrganizationalDelta, ProgramOrganizationalPatchEntry};
use crate::artifacts::program::registers::OrganizationalRequirementPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameOrganizationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = OrganizationalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { organizational: Some(ProgramOrganizationalDelta { patched: vec![ProgramOrganizationalPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
