//! 🦠️ ProgramSnapshot mutation — `replace-sustainability-requirement` leaf (replace). Split from the
//! pre-migration `♻️sustainability` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::SustainabilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one sustainability requirement row's non-identity content, addressed by
/// `sustainability_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSustainabilityRequirement {
    pub sustainability_requirement: SustainabilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSustainabilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "sustainability-requirement", kind: "replace-sustainability-requirement", record: "ReplacedSustainabilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace sustainability requirement \"{}\"", self.sustainability_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.sustainability_requirement.header.id.0.clone()]
    }
}
