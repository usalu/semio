//! 🔺️ Sparse diff construction for the `replace-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::mutation::ReplaceResource;
use crate::artifacts::program::diff::{ProgramResourcesDelta, ProgramResourcesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.resources.iter().find(|row| row.header.id == payload.resource.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resource exists with this id.", [payload.resource.header.id.0.clone()]);
    };
    if existing == &payload.resource {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This resource already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.resource).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(ProgramResourcesDelta { patched: vec![ProgramResourcesPatchEntry { id: payload.resource.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
