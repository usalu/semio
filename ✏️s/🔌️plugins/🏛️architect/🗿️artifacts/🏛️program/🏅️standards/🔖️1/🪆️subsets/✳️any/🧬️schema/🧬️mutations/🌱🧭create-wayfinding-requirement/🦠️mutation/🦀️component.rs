//! 🦠️ ProgramSnapshot mutation — `create-wayfinding-requirement` leaf (create). Split from the
//! pre-migration `🧭wayfinding` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::WayfindingRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new wayfinding requirement row into existence in `program.wayfinding`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWayfindingRequirement {
    pub wayfinding_requirement: WayfindingRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateWayfindingRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wayfinding-requirement", kind: "create-wayfinding-requirement", record: "CreatedWayfindingRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create wayfinding requirement \"{}\"", self.wayfinding_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.wayfinding_requirement.header.id.0.clone()]
    }
}
