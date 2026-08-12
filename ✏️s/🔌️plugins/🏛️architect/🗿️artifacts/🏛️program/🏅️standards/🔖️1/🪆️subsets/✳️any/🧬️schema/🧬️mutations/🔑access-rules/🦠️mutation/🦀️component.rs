//! 🦠️ ProgramSnapshot mutation — `access_rules` leaf: create/delete/rename/replace access rule rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `AccessRule` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::AccessRule;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateAccessRule
/// 🌱️ Brings a new access rule row into existence in `program.access_rules`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccessRule {
    pub access_rule: AccessRule,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAccessRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "access-rule", kind: "create-access-rule", record: "CreatedAccessRule" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create access rule \"{}\"", self.access_rule.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.access_rule.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateAccessRule

//#region 🔖️DeleteAccessRule
/// 🗑️ Removes a access rule row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAccessRule {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteAccessRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "access-rule", kind: "delete-access-rule", record: "DeletedAccessRule" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete access rule \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteAccessRule

//#region 🔖️RenameAccessRule
/// ✏️ Sets the identity `name` field of one access rule row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameAccessRule {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameAccessRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "access-rule", kind: "rename-access-rule", record: "RenamedAccessRule" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename access rule to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameAccessRule

//#region 🔖️ReplaceAccessRule
/// 🔁️ Whole-value swap of one access rule row's non-identity content, addressed by
/// `access_rule.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAccessRule {
    pub access_rule: AccessRule,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceAccessRule {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "access-rule", kind: "replace-access-rule", record: "ReplacedAccessRule" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace access rule \"{}\"", self.access_rule.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.access_rule.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceAccessRule
