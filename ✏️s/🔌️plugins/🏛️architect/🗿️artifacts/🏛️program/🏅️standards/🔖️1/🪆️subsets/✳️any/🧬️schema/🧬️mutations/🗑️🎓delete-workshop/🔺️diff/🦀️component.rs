//! 🔺️ Sparse diff construction for the `delete-workshop` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🎓workshops` per Wave C.

use super::mutation::DeleteWorkshop;
use crate::artifacts::program::diff::ProgramWorkshopsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteWorkshop, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.workshops.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No workshop exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { workshops: Some(ProgramWorkshopsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
