//! 🔺️ Sparse diff construction for the `delete-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::DeleteRegulatoryRequirement;
use crate::diff::ProgramRegulatoryDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteRegulatoryRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.regulatory.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No regulatory requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
