//! 🔺️ Sparse diff construction for the `create-document` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📄documents` per Wave C.

use super::CreateDocument;
use crate::diff::ProgramArtifactsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateDocument, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.document.header.id;
    if base.artifacts.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A document already exists with this id.", [id.0.clone()]);
    }
    let length = base.artifacts.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the document list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.artifacts.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { artifacts: Some(ProgramArtifactsDelta { added: vec![payload.document.clone()], reordered, ..Default::default() }), ..Default::default() })
}
