//! 🦠️ ProgramSnapshot mutation — `replace-relationship` leaf (replace). Split from the
//! pre-migration `🔗relationships` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::Relationship;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one relationship row's non-identity content, addressed by
/// `relationship.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceRelationship {
    pub relationship: Relationship,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "relationship", kind: "replace-relationship", record: "ReplacedRelationship" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace relationship \"{}\"", self.relationship.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.relationship.header.id.0.clone()]
    }
}
