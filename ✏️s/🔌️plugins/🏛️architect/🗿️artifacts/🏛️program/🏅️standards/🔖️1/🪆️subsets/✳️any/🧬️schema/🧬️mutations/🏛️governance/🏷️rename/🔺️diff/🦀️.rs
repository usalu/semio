//! 🔺️ Sparse diff construction for the `rename-governance` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏛️update-governance` per Wave C.

use super::RenameGovernance;
use crate::diff::GovernanceEdit;
use crate::registers::GovernancePatch;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// ✏️ Patches only `framework` of the section. Root-scoped singleton — always present, so
/// Warning `mutation.no-op` (empty diff) covers the only degenerate case: the framework is
/// unchanged.
pub fn diff(payload: &RenameGovernance, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.governance.framework == payload.new_framework {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "Governance framework already has this value.").at([base.governance.id.0.clone()])]);
    }
    protocol::MutationOutcome::new(ProgramDiff { governance: Some(GovernanceEdit::patching(GovernancePatch { framework: Some(payload.new_framework.clone()), ..Default::default() })), ..Default::default() })
}
