//! 🦠️ ProgramSnapshot mutation — `create-regulatory-requirement` leaf (create). Split from the
//! pre-migration `📜regulatory` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::RegulatoryRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new regulatory requirement row into existence in `program.regulatory`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegulatoryRequirement {
    pub regulatory_requirement: RegulatoryRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateRegulatoryRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "regulatory-requirement", kind: "create-regulatory-requirement", record: "CreatedRegulatoryRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create regulatory requirement \"{}\"", self.regulatory_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.regulatory_requirement.header.id.0.clone()]
    }
}
