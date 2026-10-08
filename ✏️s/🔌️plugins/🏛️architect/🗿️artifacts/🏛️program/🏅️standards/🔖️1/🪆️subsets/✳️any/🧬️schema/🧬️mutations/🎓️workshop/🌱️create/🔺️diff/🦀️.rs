//! 🔺️ Sparse diff construction for the `create-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::CreateWorkshop;
use crate::diff::ProgramWorkshopsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateWorkshop, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.workshop.header.id;
    if base.workshops.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A workshop already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.workshops.len());
    if at > base.workshops.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the workshop list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { workshops: Some(ProgramWorkshopsDelta::insertion(at, payload.workshop.clone())), ..Default::default() })
}
