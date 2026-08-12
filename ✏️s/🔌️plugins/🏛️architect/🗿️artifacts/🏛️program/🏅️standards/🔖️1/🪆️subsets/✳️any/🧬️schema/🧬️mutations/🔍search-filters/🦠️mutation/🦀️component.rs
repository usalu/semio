//! 🦠️ ProgramSnapshot mutation — `search_filters` leaf: create/delete/rename/replace search filter rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `SearchFilter` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::SearchFilter;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateSearchFilter
/// 🌱️ Brings a new search filter row into existence in `program.search_filters`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSearchFilter {
    pub search_filter: SearchFilter,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSearchFilter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "search-filter", kind: "create-search-filter", record: "CreatedSearchFilter" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create search filter \"{}\"", self.search_filter.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.search_filter.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateSearchFilter

//#region 🔖️DeleteSearchFilter
/// 🗑️ Removes a search filter row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSearchFilter {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteSearchFilter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "search-filter", kind: "delete-search-filter", record: "DeletedSearchFilter" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete search filter \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteSearchFilter

//#region 🔖️RenameSearchFilter
/// ✏️ Sets the identity `name` field of one search filter row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSearchFilter {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameSearchFilter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "search-filter", kind: "rename-search-filter", record: "RenamedSearchFilter" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename search filter to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameSearchFilter

//#region 🔖️ReplaceSearchFilter
/// 🔁️ Whole-value swap of one search filter row's non-identity content, addressed by
/// `search_filter.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSearchFilter {
    pub search_filter: SearchFilter,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSearchFilter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "search-filter", kind: "replace-search-filter", record: "ReplacedSearchFilter" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace search filter \"{}\"", self.search_filter.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.search_filter.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceSearchFilter
