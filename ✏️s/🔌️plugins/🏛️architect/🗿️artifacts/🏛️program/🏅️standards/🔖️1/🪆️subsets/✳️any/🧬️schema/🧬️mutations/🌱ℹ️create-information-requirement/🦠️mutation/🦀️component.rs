//! 🦠️ ProgramSnapshot mutation — `create-information-requirement` leaf (create). Split from the
//! pre-migration `ℹ️information` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::InformationRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new information requirement row into existence in `program.information`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInformationRequirement {
    pub information_requirement: InformationRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateInformationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "information-requirement", kind: "create-information-requirement", record: "CreatedInformationRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create information requirement \"{}\"", self.information_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.information_requirement.header.id.0.clone()]
    }
}
