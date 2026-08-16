//! 🦠️ ProgramSnapshot mutation — `create-program-element` leaf (create). Split from the
//! pre-migration `🧱elements` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ProgramElement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new program element row into existence in `program.elements`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProgramElement {
    pub program_element: ProgramElement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateProgramElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "program-element", kind: "create-program-element", record: "CreatedProgramElement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create program element \"{}\"", self.program_element.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.program_element.header.id.0.clone()]
    }
}
