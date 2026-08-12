//! 🦠️ ProgramSnapshot mutation — `users` leaf: create/delete/rename/replace user profile rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `UserProfile` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::UserProfile;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateUserProfile
/// 🌱️ Brings a new user profile row into existence in `program.users`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserProfile {
    pub user_profile: UserProfile,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateUserProfile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "user-profile", kind: "create-user-profile", record: "CreatedUserProfile" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create user profile \"{}\"", self.user_profile.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.user_profile.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateUserProfile

//#region 🔖️DeleteUserProfile
/// 🗑️ Removes a user profile row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserProfile {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteUserProfile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "user-profile", kind: "delete-user-profile", record: "DeletedUserProfile" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete user profile \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteUserProfile

//#region 🔖️RenameUserProfile
/// ✏️ Sets the identity `name` field of one user profile row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameUserProfile {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameUserProfile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "user-profile", kind: "rename-user-profile", record: "RenamedUserProfile" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename user profile to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameUserProfile

//#region 🔖️ReplaceUserProfile
/// 🔁️ Whole-value swap of one user profile row's non-identity content, addressed by
/// `user_profile.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceUserProfile {
    pub user_profile: UserProfile,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceUserProfile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "user-profile", kind: "replace-user-profile", record: "ReplacedUserProfile" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace user profile \"{}\"", self.user_profile.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.user_profile.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceUserProfile
