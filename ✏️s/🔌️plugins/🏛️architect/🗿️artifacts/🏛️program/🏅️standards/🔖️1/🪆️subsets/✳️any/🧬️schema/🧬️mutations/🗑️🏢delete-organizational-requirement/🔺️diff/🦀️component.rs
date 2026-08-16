//! 🔺️ Sparse diff construction for the `delete-organizational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏢organizational` per Wave C.

use super::mutation::DeleteOrganizationalRequirement;
use crate::artifacts::program::diff::ProgramOrganizationalDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteOrganizationalRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { organizational: Some(ProgramOrganizationalDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
