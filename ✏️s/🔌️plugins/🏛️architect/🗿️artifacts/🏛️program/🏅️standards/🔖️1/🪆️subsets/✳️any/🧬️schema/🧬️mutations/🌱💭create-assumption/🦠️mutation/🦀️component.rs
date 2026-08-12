//! 🦠️ ProgramSnapshot mutation — `create-assumption` leaf (create). Split from the
//! pre-migration `💭assumptions` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::Assumption;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new assumption row into existence in `program.assumptions`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssumption {
    pub assumption: Assumption,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAssumption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "assumption", kind: "create-assumption", record: "CreatedAssumption" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create assumption \"{}\"", self.assumption.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.assumption.header.id.0.clone()]
    }
}
