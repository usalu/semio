//! 🦠️ ProgramSnapshot mutation — `create-infrastructure-requirement` leaf (create). Split from the
//! pre-migration `🏗️infrastructure` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::InfrastructureRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new infrastructure requirement row into existence in `program.infrastructure`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInfrastructureRequirement {
    pub infrastructure_requirement: InfrastructureRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateInfrastructureRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "infrastructure-requirement", kind: "create-infrastructure-requirement", record: "CreatedInfrastructureRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create infrastructure requirement \"{}\"", self.infrastructure_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.infrastructure_requirement.header.id.0.clone()]
    }
}
