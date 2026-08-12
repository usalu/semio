//! 🦠️ ProgramSnapshot mutation — `benchmarks` leaf: create/delete/rename/replace benchmark record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `BenchmarkRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::BenchmarkRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateBenchmarkRecord
/// 🌱️ Brings a new benchmark record row into existence in `program.benchmarks`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBenchmarkRecord {
    pub benchmark_record: BenchmarkRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateBenchmarkRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "benchmark-record", kind: "create-benchmark-record", record: "CreatedBenchmarkRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create benchmark record \"{}\"", self.benchmark_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.benchmark_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateBenchmarkRecord

//#region 🔖️DeleteBenchmarkRecord
/// 🗑️ Removes a benchmark record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteBenchmarkRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteBenchmarkRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "benchmark-record", kind: "delete-benchmark-record", record: "DeletedBenchmarkRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete benchmark record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteBenchmarkRecord

//#region 🔖️RenameBenchmarkRecord
/// ✏️ Sets the identity `name` field of one benchmark record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameBenchmarkRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameBenchmarkRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "benchmark-record", kind: "rename-benchmark-record", record: "RenamedBenchmarkRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename benchmark record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameBenchmarkRecord

//#region 🔖️ReplaceBenchmarkRecord
/// 🔁️ Whole-value swap of one benchmark record row's non-identity content, addressed by
/// `benchmark_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceBenchmarkRecord {
    pub benchmark_record: BenchmarkRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceBenchmarkRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "benchmark-record", kind: "replace-benchmark-record", record: "ReplacedBenchmarkRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace benchmark record \"{}\"", self.benchmark_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.benchmark_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceBenchmarkRecord
