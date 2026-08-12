//! 🦠️ ProgramSnapshot mutation — `relationships` leaf: create/delete/rename/replace relationship rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Relationship` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Relationship;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateRelationship
/// 🌱️ Brings a new relationship row into existence in `program.relationships`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRelationship {
    pub relationship: Relationship,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "relationship", kind: "create-relationship", record: "CreatedRelationship" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create relationship \"{}\"", self.relationship.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.relationship.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateRelationship

//#region 🔖️DeleteRelationship
/// 🗑️ Removes a relationship row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRelationship {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "relationship", kind: "delete-relationship", record: "DeletedRelationship" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete relationship \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteRelationship

//#region 🔖️RenameRelationship
/// ✏️ Sets the identity `name` field of one relationship row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameRelationship {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "relationship", kind: "rename-relationship", record: "RenamedRelationship" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename relationship to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameRelationship

//#region 🔖️ReplaceRelationship
/// 🔁️ Whole-value swap of one relationship row's non-identity content, addressed by
/// `relationship.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceRelationship {
    pub relationship: Relationship,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "relationship", kind: "replace-relationship", record: "ReplacedRelationship" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace relationship \"{}\"", self.relationship.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.relationship.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceRelationship
