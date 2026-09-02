//! 🦠️ ProgramSnapshot mutation — `replace-activity` leaf (replace). Split from the
//! pre-migration `🏃activities` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Activity;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔁️ Whole-value swap of one activity row's non-identity content, addressed by
/// `activity.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReplaceActivity {
    pub activity: Activity,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceActivity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "activity", kind: "replace-activity", record: "ReplacedActivity" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace activity \"{}\"", self.activity.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.activity.header.id.0.clone()]
    }
}
