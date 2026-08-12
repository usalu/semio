//! 🦠️ ProgramSnapshot mutation — `replace-document` leaf (replace). Split from the
//! pre-migration `📄documents` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::ArtifactRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one document row's non-identity content, addressed by
/// `document.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceDocument {
    pub document: ArtifactRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "document", kind: "replace-document", record: "ReplacedDocument" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace document \"{}\"", self.document.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.document.header.id.0.clone()]
    }
}
