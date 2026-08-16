//! 🦠️ ProgramSnapshot mutation — `create-analysis-record` leaf (create). Split from the
//! pre-migration `🔬analyses` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::AnalysisRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new analysis record row into existence in `program.analyses`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAnalysisRecord {
    pub analysis_record: AnalysisRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAnalysisRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "analysis-record", kind: "create-analysis-record", record: "CreatedAnalysisRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create analysis record \"{}\"", self.analysis_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.analysis_record.header.id.0.clone()]
    }
}
