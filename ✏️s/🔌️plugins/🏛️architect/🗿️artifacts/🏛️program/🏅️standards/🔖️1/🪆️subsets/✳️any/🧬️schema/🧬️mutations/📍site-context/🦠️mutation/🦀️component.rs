//! 🦠️ ProgramSnapshot mutation — `site_context` leaf: create/delete/rename/replace site context rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `SiteContext` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::SiteContext;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateSiteContext
/// 🌱️ Brings a new site context row into existence in `program.site_context`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSiteContext {
    pub site_context: SiteContext,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSiteContext {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "site-context", kind: "create-site-context", record: "CreatedSiteContext" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create site context \"{}\"", self.site_context.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.site_context.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateSiteContext

//#region 🔖️DeleteSiteContext
/// 🗑️ Removes a site context row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSiteContext {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteSiteContext {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "site-context", kind: "delete-site-context", record: "DeletedSiteContext" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete site context \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteSiteContext

//#region 🔖️RenameSiteContext
/// ✏️ Sets the identity `name` field of one site context row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSiteContext {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameSiteContext {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "site-context", kind: "rename-site-context", record: "RenamedSiteContext" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename site context to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameSiteContext

//#region 🔖️ReplaceSiteContext
/// 🔁️ Whole-value swap of one site context row's non-identity content, addressed by
/// `site_context.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSiteContext {
    pub site_context: SiteContext,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSiteContext {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "site-context", kind: "replace-site-context", record: "ReplacedSiteContext" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace site context \"{}\"", self.site_context.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.site_context.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceSiteContext
