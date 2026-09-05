//! 🦠️ ProgramSnapshot mutation — `replace-function` leaf (replace). Split from the
//! pre-migration `⚙️functions` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Function;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔁️ Whole-value swap of one function row's non-identity content, addressed by
/// `function.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReplaceFunction {
    pub function: Function,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceFunction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "function", kind: "replace-function", record: "ReplacedFunction" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace function \"{}\"", self.function.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.function.header.id.0.clone()]
    }
}
