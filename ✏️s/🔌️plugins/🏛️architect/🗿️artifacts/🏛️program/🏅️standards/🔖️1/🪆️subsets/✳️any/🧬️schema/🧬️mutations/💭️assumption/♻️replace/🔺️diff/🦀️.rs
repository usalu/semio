//! 🔺️ Sparse diff construction for the `replace-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::ReplaceAssumption;
use crate::artifacts::program::diff::{ProgramAssumptionsDelta, ProgramAssumptionsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceAssumption, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.assumptions.iter().find(|row| row.header.id == payload.assumption.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No assumption exists with this id.", [payload.assumption.header.id.0.clone()]);
    };
    if existing == &payload.assumption {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This assumption already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.assumption).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { patched: vec![ProgramAssumptionsPatchEntry { id: payload.assumption.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
