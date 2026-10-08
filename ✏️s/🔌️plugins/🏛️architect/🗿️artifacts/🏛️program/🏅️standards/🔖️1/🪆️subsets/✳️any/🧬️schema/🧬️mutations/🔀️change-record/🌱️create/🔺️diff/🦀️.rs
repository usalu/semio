//! 🔺️ Sparse diff construction for the `create-change-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📝changes` per Wave C.

use super::CreateChangeRecord;
use crate::diff::ProgramChangesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateChangeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.change_record.header.id;
    if base.changes.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A change record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.changes.len());
    if at > base.changes.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the change record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { changes: Some(ProgramChangesDelta::insertion(at, payload.change_record.clone())), ..Default::default() })
}
