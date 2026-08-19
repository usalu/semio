//! 🦠️ ProgramSnapshot mutation — `replace-program-element` leaf (replace). Split from the
//! pre-migration `🧱elements` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ProgramElement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one program element row's non-identity content, addressed by
/// `program_element.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceProgramElement {
    pub program_element: ProgramElement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceProgramElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "program-element", kind: "replace-program-element", record: "ReplacedProgramElement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace program element \"{}\"", self.program_element.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.program_element.header.id.0.clone()]
    }
}
