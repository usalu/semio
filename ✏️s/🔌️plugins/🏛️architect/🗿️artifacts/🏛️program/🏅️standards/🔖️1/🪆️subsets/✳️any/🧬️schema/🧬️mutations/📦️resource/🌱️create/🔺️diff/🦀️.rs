//! 🔺️ Sparse diff construction for the `create-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::CreateResource;
use crate::diff::ProgramResourcesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.resource.header.id;
    if base.resources.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A resource already exists with this id.", [id.0.clone()]);
    }
    let length = base.resources.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the resource list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.resources.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(ProgramResourcesDelta { added: vec![payload.resource.clone()], reordered, ..Default::default() }), ..Default::default() })
}
