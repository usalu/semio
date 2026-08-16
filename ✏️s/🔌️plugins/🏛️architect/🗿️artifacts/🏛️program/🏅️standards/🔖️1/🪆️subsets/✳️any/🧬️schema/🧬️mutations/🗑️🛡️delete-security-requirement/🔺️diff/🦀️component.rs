//! 🔺️ Sparse diff construction for the `delete-security-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛡️security` per Wave C.

use super::mutation::DeleteSecurityRequirement;
use crate::artifacts::program::diff::ProgramSecurityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteSecurityRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { security: Some(ProgramSecurityDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
