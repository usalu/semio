//! 🦠️ ProgramSnapshot mutation — `create-access-rule` leaf (create). Split from the
//! pre-migration `🔑access-rules` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::AccessRule;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new access rule row into existence in `program.access_rules`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateAccessRule {
    pub access_rule: AccessRule,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAccessRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "access-rule", kind: "create-access-rule", record: "CreatedAccessRule" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create access rule \"{}\"", self.access_rule.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.access_rule.header.id.0.clone()]
    }
}
