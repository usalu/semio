//! 🦠️ ProgramSnapshot mutation — `create-flexibility-requirement` leaf (create). Split from the
//! pre-migration `🧩flexibility` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::FlexibilityRequirement;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new flexibility requirement row into existence in `program.flexibility`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFlexibilityRequirement {
    pub flexibility_requirement: FlexibilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateFlexibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "flexibility-requirement", kind: "create-flexibility-requirement", record: "CreatedFlexibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create flexibility requirement \"{}\"", self.flexibility_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.flexibility_requirement.header.id.0.clone()]
    }
}
