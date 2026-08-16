//! 🦠️ ProgramSnapshot mutation — `create-quality-record` leaf (create). Split from the
//! pre-migration `💎quality` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::QualityRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new quality record row into existence in `program.quality`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQualityRecord {
    pub quality_record: QualityRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateQualityRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "quality-record", kind: "create-quality-record", record: "CreatedQualityRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create quality record \"{}\"", self.quality_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.quality_record.header.id.0.clone()]
    }
}
