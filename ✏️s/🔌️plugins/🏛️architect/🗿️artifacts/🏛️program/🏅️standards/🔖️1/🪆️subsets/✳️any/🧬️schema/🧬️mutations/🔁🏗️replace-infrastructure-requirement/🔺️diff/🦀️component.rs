//! 🔺️ Sparse diff construction for the `replace-infrastructure-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏗️infrastructure` per Wave C.

use super::mutation::ReplaceInfrastructureRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramInfrastructureDelta, ProgramInfrastructurePatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceInfrastructureRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.infrastructure.iter().find(|row| row.header.id == payload.infrastructure_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.infrastructure_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { infrastructure: Some(ProgramInfrastructureDelta { patched: vec![ProgramInfrastructurePatchEntry { id: payload.infrastructure_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
