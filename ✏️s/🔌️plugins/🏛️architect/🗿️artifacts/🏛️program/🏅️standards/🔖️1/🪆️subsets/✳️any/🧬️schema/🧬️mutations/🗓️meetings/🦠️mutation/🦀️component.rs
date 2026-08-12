//! 🦠️ ProgramSnapshot mutation — `meetings` leaf: create/delete/rename/replace meeting record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `MeetingRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::MeetingRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateMeetingRecord
/// 🌱️ Brings a new meeting record row into existence in `program.meetings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMeetingRecord {
    pub meeting_record: MeetingRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateMeetingRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "meeting-record", kind: "create-meeting-record", record: "CreatedMeetingRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create meeting record \"{}\"", self.meeting_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.meeting_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateMeetingRecord

//#region 🔖️DeleteMeetingRecord
/// 🗑️ Removes a meeting record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMeetingRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteMeetingRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "meeting-record", kind: "delete-meeting-record", record: "DeletedMeetingRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete meeting record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteMeetingRecord

//#region 🔖️RenameMeetingRecord
/// ✏️ Sets the identity `name` field of one meeting record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameMeetingRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameMeetingRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "meeting-record", kind: "rename-meeting-record", record: "RenamedMeetingRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename meeting record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameMeetingRecord

//#region 🔖️ReplaceMeetingRecord
/// 🔁️ Whole-value swap of one meeting record row's non-identity content, addressed by
/// `meeting_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceMeetingRecord {
    pub meeting_record: MeetingRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceMeetingRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "meeting-record", kind: "replace-meeting-record", record: "ReplacedMeetingRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace meeting record \"{}\"", self.meeting_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.meeting_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceMeetingRecord
