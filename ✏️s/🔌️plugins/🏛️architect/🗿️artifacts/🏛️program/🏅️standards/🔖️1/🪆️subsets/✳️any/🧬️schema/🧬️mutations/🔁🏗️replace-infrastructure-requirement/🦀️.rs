//! 🦠️ ProgramSnapshot mutation — `replace-infrastructure-requirement` leaf (replace). Split from the
//! pre-migration `🏗️infrastructure` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::InfrastructureRequirement;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one infrastructure requirement row's non-identity content, addressed by
/// `infrastructure_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceInfrastructureRequirement {
    pub infrastructure_requirement: InfrastructureRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceInfrastructureRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "infrastructure-requirement", kind: "replace-infrastructure-requirement", record: "ReplacedInfrastructureRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace infrastructure requirement \"{}\"", self.infrastructure_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.infrastructure_requirement.header.id.0.clone()]
    }
}
