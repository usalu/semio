//! 🔺️ Sparse diff construction for the `replace-storage-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗄️storage` per Wave C.

use super::ReplaceStorageRequirement;
use crate::diff::ProgramStorageDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceStorageRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.storage_requirement.header.id;
    let Some(position) = base.storage.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No storage requirement exists with this id.", [id.0.clone()]);
    };
    if base.storage[position] == payload.storage_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This storage requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.storage.len()).then(|| base.storage.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { storage: Some(ProgramStorageDelta { removed: vec![id.0.clone()], added: vec![payload.storage_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
