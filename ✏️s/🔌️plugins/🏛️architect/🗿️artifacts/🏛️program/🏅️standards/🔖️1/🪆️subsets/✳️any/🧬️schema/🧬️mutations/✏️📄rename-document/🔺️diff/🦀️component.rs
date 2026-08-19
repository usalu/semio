//! 🔺️ Sparse diff construction for the `rename-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::mutation::RenameDocument;
use crate::artifacts::program::diff::{ProgramArtifactsDelta, ProgramArtifactsPatchEntry};
use crate::artifacts::program::registers::ArtifactRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameDocument, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.artifacts.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No document exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This document already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ArtifactRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { documents: Some(ProgramArtifactsDelta { patched: vec![ProgramArtifactsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
