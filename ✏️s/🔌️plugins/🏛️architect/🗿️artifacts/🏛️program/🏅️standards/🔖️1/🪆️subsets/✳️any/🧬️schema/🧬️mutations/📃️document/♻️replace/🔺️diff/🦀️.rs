//! 🔺️ Sparse diff construction for the `replace-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::ReplaceDocument;
use crate::diff::ProgramArtifactsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceDocument, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.document.header.id;
    let Some(position) = base.artifacts.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No document exists with this id.", [id.0.clone()]);
    };
    if base.artifacts[position] == payload.document {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This document already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramArtifactsDelta::removal(&base.artifacts, position);
    delta.absorb(ProgramArtifactsDelta::insertion(position, payload.document.clone()));
    protocol::MutationOutcome::new(ProgramDiff { artifacts: Some(delta), ..Default::default() })
}
