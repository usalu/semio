//! 🔺️ Sparse diff construction for the `create-accessibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♿accessibility` per Wave C.

use super::CreateAccessibilityRequirement;
use crate::diff::ProgramAccessibilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateAccessibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.accessibility_requirement.header.id;
    if base.accessibility.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An accessibility requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.accessibility.len());
    if at > base.accessibility.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the accessibility requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { accessibility: Some(ProgramAccessibilityDelta::insertion(at, payload.accessibility_requirement.clone())), ..Default::default() })
}
