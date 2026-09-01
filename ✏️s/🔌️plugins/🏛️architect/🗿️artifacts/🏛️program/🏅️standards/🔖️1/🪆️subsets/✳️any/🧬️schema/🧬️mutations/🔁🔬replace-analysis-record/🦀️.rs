//! 🦠️ ProgramSnapshot mutation — `replace-analysis-record` leaf (replace). Split from the
//! pre-migration `🔬analyses` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::AnalysisRecord;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one analysis record row's non-identity content, addressed by
/// `analysis_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAnalysisRecord {
    pub analysis_record: AnalysisRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceAnalysisRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "analysis-record", kind: "replace-analysis-record", record: "ReplacedAnalysisRecord" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace analysis record \"{}\"", self.analysis_record.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.analysis_record.header.id.0.clone()]
    }
}
