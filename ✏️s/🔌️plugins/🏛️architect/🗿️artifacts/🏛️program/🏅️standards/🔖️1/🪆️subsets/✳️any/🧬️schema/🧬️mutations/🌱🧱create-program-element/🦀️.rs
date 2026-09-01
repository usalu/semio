//! 🦠️ ProgramSnapshot mutation — `create-program-element` leaf (create). Split from the
//! pre-migration `🧱elements` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ProgramElement;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new program element row into existence in `program.elements`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct CreateProgramElement {
    pub program_element: ProgramElement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateProgramElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "program-element", kind: "create-program-element", record: "CreatedProgramElement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create program element \"{}\"", self.program_element.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.program_element.header.id.0.clone()]
    }
}
