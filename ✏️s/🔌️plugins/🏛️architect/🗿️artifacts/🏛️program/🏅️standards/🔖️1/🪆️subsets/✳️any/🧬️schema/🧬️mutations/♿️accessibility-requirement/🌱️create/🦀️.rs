//! 🦠️ ProgramSnapshot mutation — `create-accessibility-requirement` leaf (create). Split from the
//! pre-migration `♿accessibility` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::AccessibilityRequirement;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new accessibility requirement row into existence in `program.accessibility`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateAccessibilityRequirement {
    pub accessibility_requirement: AccessibilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAccessibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "accessibility-requirement", kind: "create-accessibility-requirement", record: "CreatedAccessibilityRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create accessibility requirement \"{}\"", self.accessibility_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.accessibility_requirement.header.id.0.clone()]
    }
}
