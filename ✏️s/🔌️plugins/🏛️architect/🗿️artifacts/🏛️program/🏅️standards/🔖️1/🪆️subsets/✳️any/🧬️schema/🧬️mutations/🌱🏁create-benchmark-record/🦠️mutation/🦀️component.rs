//! 🦠️ ProgramSnapshot mutation — `create-benchmark-record` leaf (create). Split from the
//! pre-migration `🏁benchmarks` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::BenchmarkRecord;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new benchmark record row into existence in `program.benchmarks`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBenchmarkRecord {
    pub benchmark_record: BenchmarkRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateBenchmarkRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "benchmark-record", kind: "create-benchmark-record", record: "CreatedBenchmarkRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create benchmark record \"{}\"", self.benchmark_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.benchmark_record.header.id.0.clone()]
    }
}
