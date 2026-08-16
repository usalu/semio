//! 🦠️ ProgramSnapshot mutation — `replace-environmental-requirement` leaf (replace). Split from the
//! pre-migration `🌿environmental` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::EnvironmentalRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one environmental requirement row's non-identity content, addressed by
/// `environmental_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceEnvironmentalRequirement {
    pub environmental_requirement: EnvironmentalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceEnvironmentalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "environmental-requirement", kind: "replace-environmental-requirement", record: "ReplacedEnvironmentalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace environmental requirement \"{}\"", self.environmental_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.environmental_requirement.header.id.0.clone()]
    }
}
