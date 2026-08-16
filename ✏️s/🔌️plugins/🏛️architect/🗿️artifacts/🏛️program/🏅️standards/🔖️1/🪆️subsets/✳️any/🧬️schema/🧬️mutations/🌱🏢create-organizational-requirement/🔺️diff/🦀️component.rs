//! 🔺️ Sparse diff construction for the `create-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::mutation::CreateOrganizationalRequirement;
use crate::artifacts::program::diff::ProgramOrganizationalDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.organizational` on apply.
pub fn diff(payload: &CreateOrganizationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { organizational: Some(ProgramOrganizationalDelta { added: vec![payload.organizational_requirement.clone()], ..Default::default() }), ..Default::default() }
}
