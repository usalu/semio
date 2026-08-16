//! 🔺️ Sparse diff construction for the `rename-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::mutation::RenameConstraintRecord;
use crate::artifacts::program::diff::{ProgramConstraintsDelta, ProgramConstraintsPatchEntry};
use crate::artifacts::program::registers::ConstraintRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameConstraintRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.constraints.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No constraint record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This constraint record already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = ConstraintRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { constraints: Some(ProgramConstraintsDelta { patched: vec![ProgramConstraintsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
