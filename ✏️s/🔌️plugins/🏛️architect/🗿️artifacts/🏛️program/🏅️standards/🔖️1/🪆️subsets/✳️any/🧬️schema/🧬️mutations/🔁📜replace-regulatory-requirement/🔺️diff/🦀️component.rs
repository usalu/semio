//! 🔺️ Sparse diff construction for the `replace-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::mutation::ReplaceRegulatoryRequirement;
use crate::artifacts::program::diff::{ProgramRegulatoryDelta, ProgramRegulatoryPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceRegulatoryRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.regulatory.iter().find(|row| row.header.id == payload.regulatory_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No regulatory requirement exists with this id.", [payload.regulatory_requirement.header.id.0.clone()]);
    };
    if existing == &payload.regulatory_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This regulatory requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.regulatory_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { patched: vec![ProgramRegulatoryPatchEntry { id: payload.regulatory_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
