//! 🦠️ ProgramSnapshot mutation — `replace-delivery-constraint` leaf (replace). Split from the
//! pre-migration `🚚delivery` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::DeliveryConstraint;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one delivery constraint row's non-identity content, addressed by
/// `delivery_constraint.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceDeliveryConstraint {
    pub delivery_constraint: DeliveryConstraint,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceDeliveryConstraint {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "delivery-constraint", kind: "replace-delivery-constraint", record: "ReplacedDeliveryConstraint" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace delivery constraint \"{}\"", self.delivery_constraint.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.delivery_constraint.header.id.0.clone()]
    }
}
