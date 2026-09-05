//! 🔺️ Sparse diff construction for the `rename-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::RenameRegulatoryRequirement;
use crate::artifacts::program::diff::{ProgramRegulatoryDelta, ProgramRegulatoryPatchEntry};
use crate::artifacts::program::registers::RegulatoryRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameRegulatoryRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.regulatory.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No regulatory requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This regulatory requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = RegulatoryRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { patched: vec![ProgramRegulatoryPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
