//! 🦠️ ProgramSnapshot mutation — `replace-quantity-requirement` leaf (replace). Split from the
//! pre-migration `🔢quantities` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::QuantityRequirement;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one quantity requirement row's non-identity content, addressed by
/// `quantity_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceQuantityRequirement {
    pub quantity_requirement: QuantityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceQuantityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "quantity-requirement", kind: "replace-quantity-requirement", record: "ReplacedQuantityRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace quantity requirement \"{}\"", self.quantity_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.quantity_requirement.header.id.0.clone()]
    }
}
