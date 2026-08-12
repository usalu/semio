//! 🦠️ ProgramSnapshot mutation — `create-relationship` leaf (create). Split from the
//! pre-migration `🔗relationships` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::Relationship;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new relationship row into existence in `program.relationships`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelationship {
    pub relationship: Relationship,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "relationship", kind: "create-relationship", record: "CreatedRelationship" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create relationship \"{}\"", self.relationship.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.relationship.header.id.0.clone()]
    }
}
