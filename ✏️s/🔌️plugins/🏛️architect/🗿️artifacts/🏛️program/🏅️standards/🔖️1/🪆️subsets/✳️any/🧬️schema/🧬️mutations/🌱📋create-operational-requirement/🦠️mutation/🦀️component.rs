//! 🦠️ ProgramSnapshot mutation — `create-operational-requirement` leaf (create). Split from the
//! pre-migration `📋operations` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::OperationalRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new operational requirement row into existence in `program.operations`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOperationalRequirement {
    pub operational_requirement: OperationalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateOperationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "operational-requirement", kind: "create-operational-requirement", record: "CreatedOperationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create operational requirement \"{}\"", self.operational_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.operational_requirement.header.id.0.clone()]
    }
}
