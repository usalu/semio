//! 🔺️ Sparse diff construction for the `create-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::mutation::CreatePrivacyRequirement;
use crate::artifacts::program::diff::ProgramPrivacyDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.privacy` on apply.
pub fn diff(payload: &CreatePrivacyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { privacy: Some(ProgramPrivacyDelta { added: vec![payload.privacy_requirement.clone()], ..Default::default() }), ..Default::default() }
}
