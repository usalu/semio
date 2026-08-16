//! 🔺️ Sparse diff construction for the `create-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::mutation::CreateSecurityRequirement;
use crate::artifacts::program::diff::ProgramSecurityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.security` on apply.
pub fn diff(payload: &CreateSecurityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { security: Some(ProgramSecurityDelta { added: vec![payload.security_requirement.clone()], ..Default::default() }), ..Default::default() }
}
