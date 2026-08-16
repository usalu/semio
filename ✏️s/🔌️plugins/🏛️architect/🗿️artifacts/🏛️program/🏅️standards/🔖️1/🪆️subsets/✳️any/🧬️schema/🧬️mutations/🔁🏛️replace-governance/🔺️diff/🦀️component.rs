//! 🔺️ Sparse diff construction for the `replace-governance` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏛️update-governance` per Wave C.

use super::mutation::ReplaceGovernance;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ New `Governance` wholesale. Root-scoped singleton — always present, so Warning
/// `mutation.no-op` (empty diff) covers the only degenerate case: the value is unchanged.
pub fn diff(payload: &ReplaceGovernance, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if base.governance == payload.new_governance {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "Governance already matches the requested value.").at([base.governance.id.0.clone()])]);
    }
    protocol::MutationOutcome::new(ProgramDiff { governance: Some(payload.new_governance.clone()), ..Default::default() })
}
