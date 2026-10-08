//! 🔺️ Sparse diff construction for the `create-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::CreateDocument;
use crate::diff::ProgramArtifactsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateDocument, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.document.header.id;
    if base.artifacts.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A document already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.artifacts.len());
    if at > base.artifacts.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the document list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { artifacts: Some(ProgramArtifactsDelta::insertion(at, payload.document.clone())), ..Default::default() })
}
