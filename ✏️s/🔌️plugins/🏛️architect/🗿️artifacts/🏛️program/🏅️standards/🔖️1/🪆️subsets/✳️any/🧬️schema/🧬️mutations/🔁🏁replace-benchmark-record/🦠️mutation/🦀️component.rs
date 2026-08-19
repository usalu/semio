//! 🦠️ ProgramSnapshot mutation — `replace-benchmark-record` leaf (replace). Split from the
//! pre-migration `🏁benchmarks` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::BenchmarkRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one benchmark record row's non-identity content, addressed by
/// `benchmark_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceBenchmarkRecord {
    pub benchmark_record: BenchmarkRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceBenchmarkRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "benchmark-record", kind: "replace-benchmark-record", record: "ReplacedBenchmarkRecord" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace benchmark record \"{}\"", self.benchmark_record.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.benchmark_record.header.id.0.clone()]
    }
}
