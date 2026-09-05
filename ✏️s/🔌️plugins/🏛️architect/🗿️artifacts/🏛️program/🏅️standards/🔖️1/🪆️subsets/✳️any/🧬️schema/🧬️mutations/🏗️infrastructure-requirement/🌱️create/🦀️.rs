//! 🦠️ ProgramSnapshot mutation — `create-infrastructure-requirement` leaf (create). Split from the
//! pre-migration `🏗️infrastructure` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::InfrastructureRequirement;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new infrastructure requirement row into existence in `program.infrastructure`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateInfrastructureRequirement {
    pub infrastructure_requirement: InfrastructureRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateInfrastructureRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "infrastructure-requirement", kind: "create-infrastructure-requirement", record: "CreatedInfrastructureRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create infrastructure requirement \"{}\"", self.infrastructure_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.infrastructure_requirement.header.id.0.clone()]
    }
}
