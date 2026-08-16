//! 🦠️ ProgramSnapshot mutation — `replace-governance` leaf (replace). Split from the
//! pre-migration `🏛️update-governance` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Governance;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of `program.governance`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceGovernance {
    pub new_governance: Governance,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceGovernance {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "governance", kind: "replace-governance", record: "ReplacedGovernance" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace governance \"{}\"", self.new_governance.framework)
    }
}
