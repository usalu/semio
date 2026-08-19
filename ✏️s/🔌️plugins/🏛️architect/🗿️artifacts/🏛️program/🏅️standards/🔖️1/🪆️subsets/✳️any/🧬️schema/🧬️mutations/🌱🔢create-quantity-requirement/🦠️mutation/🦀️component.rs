//! 🦠️ ProgramSnapshot mutation — `create-quantity-requirement` leaf (create). Split from the
//! pre-migration `🔢quantities` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::QuantityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new quantity requirement row into existence in `program.quantities`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuantityRequirement {
    pub quantity_requirement: QuantityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateQuantityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "quantity-requirement", kind: "create-quantity-requirement", record: "CreatedQuantityRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create quantity requirement \"{}\"", self.quantity_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.quantity_requirement.header.id.0.clone()]
    }
}
