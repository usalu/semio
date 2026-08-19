//! 🔺️ Sparse diff construction for the `replace-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::mutation::ReplaceDocument;
use crate::artifacts::program::diff::{ProgramArtifactsDelta, ProgramArtifactsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceDocument, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.artifacts.iter().find(|row| row.header.id == payload.document.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No document exists with this id.", [payload.document.header.id.0.clone()]);
    };
    if existing == &payload.document {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This document already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.document).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { documents: Some(ProgramArtifactsDelta { patched: vec![ProgramArtifactsPatchEntry { id: payload.document.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
