//! 🦠️ ProgramSnapshot mutation — `analyses` leaf: create/delete/rename/replace analysis record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `AnalysisRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::AnalysisRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateAnalysisRecord
/// 🌱️ Brings a new analysis record row into existence in `program.analyses`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAnalysisRecord {
    pub analysis_record: AnalysisRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAnalysisRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "analysis-record", kind: "create-analysis-record", record: "CreatedAnalysisRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create analysis record \"{}\"", self.analysis_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.analysis_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateAnalysisRecord

//#region 🔖️DeleteAnalysisRecord
/// 🗑️ Removes a analysis record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAnalysisRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteAnalysisRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "analysis-record", kind: "delete-analysis-record", record: "DeletedAnalysisRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete analysis record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteAnalysisRecord

//#region 🔖️RenameAnalysisRecord
/// ✏️ Sets the identity `name` field of one analysis record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameAnalysisRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameAnalysisRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "analysis-record", kind: "rename-analysis-record", record: "RenamedAnalysisRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename analysis record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameAnalysisRecord

//#region 🔖️ReplaceAnalysisRecord
/// 🔁️ Whole-value swap of one analysis record row's non-identity content, addressed by
/// `analysis_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAnalysisRecord {
    pub analysis_record: AnalysisRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceAnalysisRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "analysis-record", kind: "replace-analysis-record", record: "ReplacedAnalysisRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace analysis record \"{}\"", self.analysis_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.analysis_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceAnalysisRecord
