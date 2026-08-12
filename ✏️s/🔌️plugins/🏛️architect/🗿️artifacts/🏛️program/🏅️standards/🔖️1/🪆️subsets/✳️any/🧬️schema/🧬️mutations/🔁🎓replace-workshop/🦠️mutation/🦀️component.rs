//! 🦠️ ProgramSnapshot mutation — `replace-workshop` leaf (replace). Split from the
//! pre-migration `🎓workshops` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::Workshop;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one workshop row's non-identity content, addressed by
/// `workshop.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceWorkshop {
    pub workshop: Workshop,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceWorkshop {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "workshop", kind: "replace-workshop", record: "ReplacedWorkshop" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace workshop \"{}\"", self.workshop.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.workshop.header.id.0.clone()]
    }
}
