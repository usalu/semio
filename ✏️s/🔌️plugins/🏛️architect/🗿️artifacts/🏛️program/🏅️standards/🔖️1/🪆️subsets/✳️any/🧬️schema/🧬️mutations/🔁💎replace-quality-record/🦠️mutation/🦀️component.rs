//! 🦠️ ProgramSnapshot mutation — `replace-quality-record` leaf (replace). Split from the
//! pre-migration `💎quality` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::QualityRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one quality record row's non-identity content, addressed by
/// `quality_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceQualityRecord {
    pub quality_record: QualityRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceQualityRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "quality-record", kind: "replace-quality-record", record: "ReplacedQualityRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace quality record \"{}\"", self.quality_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.quality_record.header.id.0.clone()]
    }
}
