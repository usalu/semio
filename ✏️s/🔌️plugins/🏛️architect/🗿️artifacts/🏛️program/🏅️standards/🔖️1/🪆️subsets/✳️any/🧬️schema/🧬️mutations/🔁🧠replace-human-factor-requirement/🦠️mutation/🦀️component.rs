//! 🦠️ ProgramSnapshot mutation — `replace-human-factor-requirement` leaf (replace). Split from the
//! pre-migration `🧠human-factors` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::HumanFactorRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one human factor requirement row's non-identity content, addressed by
/// `human_factor_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceHumanFactorRequirement {
    pub human_factor_requirement: HumanFactorRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceHumanFactorRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "human-factor-requirement", kind: "replace-human-factor-requirement", record: "ReplacedHumanFactorRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace human factor requirement \"{}\"", self.human_factor_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.human_factor_requirement.header.id.0.clone()]
    }
}
