//! 🦠️ ProgramSnapshot mutation — `resources` leaf: create/delete/rename/replace resource rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Resource` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Resource;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateResource
/// 🌱️ Brings a new resource row into existence in `program.resources`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResource {
    pub resource: Resource,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateResource {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "resource", kind: "create-resource", record: "CreatedResource" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create resource \"{}\"", self.resource.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.resource.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateResource

//#region 🔖️DeleteResource
/// 🗑️ Removes a resource row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResource {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteResource {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "resource", kind: "delete-resource", record: "DeletedResource" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete resource \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteResource

//#region 🔖️RenameResource
/// ✏️ Sets the identity `name` field of one resource row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameResource {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameResource {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "resource", kind: "rename-resource", record: "RenamedResource" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename resource to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameResource

//#region 🔖️ReplaceResource
/// 🔁️ Whole-value swap of one resource row's non-identity content, addressed by
/// `resource.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceResource {
    pub resource: Resource,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceResource {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "resource", kind: "replace-resource", record: "ReplacedResource" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace resource \"{}\"", self.resource.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.resource.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceResource
