//! 🦠️ ProgramSnapshot mutation — `create-safety-requirement` leaf (create). Split from the
//! pre-migration `🦺safety` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::SafetyRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new safety requirement row into existence in `program.safety`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSafetyRequirement {
    pub safety_requirement: SafetyRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSafetyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "safety-requirement", kind: "create-safety-requirement", record: "CreatedSafetyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create safety requirement \"{}\"", self.safety_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.safety_requirement.header.id.0.clone()]
    }
}
