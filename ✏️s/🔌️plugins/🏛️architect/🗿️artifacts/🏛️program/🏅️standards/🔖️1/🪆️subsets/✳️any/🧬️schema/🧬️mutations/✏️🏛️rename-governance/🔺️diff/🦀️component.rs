//! 🔺️ Sparse diff construction for the `rename-governance` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏛️update-governance` per Wave C.

use super::mutation::RenameGovernance;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `Governance` with only `framework` changed. Root-scoped singleton — always present, so
/// Warning `mutation.no-op` (empty diff) covers the only degenerate case: the framework is
/// unchanged.
pub fn diff(payload: &RenameGovernance, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.governance.framework == payload.new_framework {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "Governance framework already has this value.").at([base.governance.id.0.clone()])]);
    }
    let mut value = base.governance.clone();
    value.framework = payload.new_framework.clone();
    protocol::MutationOutcome::new(ProgramDiff { governance: Some(value), ..Default::default() })
}
