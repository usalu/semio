//! 🔺️ Sparse diff construction for the `delete-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::mutation::DeletePrivacyRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramPrivacyDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeletePrivacyRequirement, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { privacy: Some(ProgramPrivacyDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
