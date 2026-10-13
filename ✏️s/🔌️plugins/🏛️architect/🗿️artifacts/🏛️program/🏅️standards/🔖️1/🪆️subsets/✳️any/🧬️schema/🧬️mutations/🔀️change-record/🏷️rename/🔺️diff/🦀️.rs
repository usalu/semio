//! 🔺️ Sparse diff construction for the `rename-change-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📝changes` per Wave C.

use super::RenameChangeRecord;
use crate::standards::v1::subsets::any::schema::diff::{ProgramChangesDelta, ProgramChangesPatchEntry};
use crate::standards::v1::subsets::any::schema::registers::ChangeRecordPatch;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `modified = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameChangeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.changes.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No change record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This change record already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ChangeRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { changes: Some(ProgramChangesDelta { modified: vec![ProgramChangesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
