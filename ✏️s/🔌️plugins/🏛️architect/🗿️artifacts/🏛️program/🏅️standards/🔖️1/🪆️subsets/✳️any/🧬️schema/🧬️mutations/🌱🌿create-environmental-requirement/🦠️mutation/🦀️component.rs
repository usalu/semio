//! 🦠️ ProgramSnapshot mutation — `create-environmental-requirement` leaf (create). Split from the
//! pre-migration `🌿environmental` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::EnvironmentalRequirement;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new environmental requirement row into existence in `program.environmental`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnvironmentalRequirement {
    pub environmental_requirement: EnvironmentalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateEnvironmentalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "environmental-requirement", kind: "create-environmental-requirement", record: "CreatedEnvironmentalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create environmental requirement \"{}\"", self.environmental_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.environmental_requirement.header.id.0.clone()]
    }
}
