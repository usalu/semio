//! 🔺️ Sparse diff construction for the `create-accessibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♿accessibility` per Wave C.

use super::mutation::CreateAccessibilityRequirement;
use crate::artifacts::program::diff::ProgramAccessibilityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateAccessibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.accessibility_requirement.header.id.clone();
    if base.accessibility.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An accessibility requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { accessibility: Some(ProgramAccessibilityDelta { added: vec![payload.accessibility_requirement.clone()], ..Default::default() }), ..Default::default() })
}
