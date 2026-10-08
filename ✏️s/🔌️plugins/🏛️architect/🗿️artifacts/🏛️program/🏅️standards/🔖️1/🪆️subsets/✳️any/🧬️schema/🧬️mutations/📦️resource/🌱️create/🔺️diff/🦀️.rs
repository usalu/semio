//! 🔺️ Sparse diff construction for the `create-resource` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📦resources` per Wave C.

use super::CreateResource;
use crate::diff::ProgramResourcesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateResource, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.resource.header.id;
    if base.resources.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A resource already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.resources.len());
    if at > base.resources.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the resource list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { resources: Some(ProgramResourcesDelta::insertion(at, payload.resource.clone())), ..Default::default() })
}
