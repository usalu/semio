//! 🦠️ ProgramSnapshot mutation — `replace-knowledge-record` leaf (replace). Split from the
//! pre-migration `📚knowledge` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::KnowledgeRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one knowledge record row's non-identity content, addressed by
/// `knowledge_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceKnowledgeRecord {
    pub knowledge_record: KnowledgeRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceKnowledgeRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "knowledge-record", kind: "replace-knowledge-record", record: "ReplacedKnowledgeRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace knowledge record \"{}\"", self.knowledge_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.knowledge_record.header.id.0.clone()]
    }
}
