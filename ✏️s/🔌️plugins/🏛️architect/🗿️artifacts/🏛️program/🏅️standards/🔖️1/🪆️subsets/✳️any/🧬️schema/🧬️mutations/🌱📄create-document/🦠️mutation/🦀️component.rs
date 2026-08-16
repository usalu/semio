//! 🦠️ ProgramSnapshot mutation — `create-document` leaf (create). Split from the
//! pre-migration `📄documents` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ArtifactRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new document row into existence in `program.artifacts`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocument {
    pub document: ArtifactRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "document", kind: "create-document", record: "CreatedDocument" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create document \"{}\"", self.document.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.document.header.id.0.clone()]
    }
}
