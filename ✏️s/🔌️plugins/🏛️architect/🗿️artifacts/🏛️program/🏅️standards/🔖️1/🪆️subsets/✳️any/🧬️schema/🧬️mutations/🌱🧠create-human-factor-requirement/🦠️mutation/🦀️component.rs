//! 🦠️ ProgramSnapshot mutation — `create-human-factor-requirement` leaf (create). Split from the
//! pre-migration `🧠human-factors` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::HumanFactorRequirement;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new human factor requirement row into existence in `program.human_factors`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHumanFactorRequirement {
    pub human_factor_requirement: HumanFactorRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateHumanFactorRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "human-factor-requirement", kind: "create-human-factor-requirement", record: "CreatedHumanFactorRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create human factor requirement \"{}\"", self.human_factor_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.human_factor_requirement.header.id.0.clone()]
    }
}
