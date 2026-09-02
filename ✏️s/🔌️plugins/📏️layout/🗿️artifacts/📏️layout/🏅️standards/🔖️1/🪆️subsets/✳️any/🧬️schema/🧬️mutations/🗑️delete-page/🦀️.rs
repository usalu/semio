//! 🗑️ `delete-page` — removes a {@link Page} by id; inverse recreates it via `create-page`.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use crate::artifacts::layout::mutations::{LayoutMutation, create_page};
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗑️DeletePage
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct DeletePage {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeletePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "page", kind: "delete-page", record: "DeletedPage" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_page(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_delete_page(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete page \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeletePage


//#region 🗑️DeletePage
pub async fn diff_delete_page(payload: &DeletePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.pages.iter().any(|page| page.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeletePage


//#region 🗑️DeletePage
pub async fn inverse_delete_page(payload: &DeletePage, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().position(|page| page.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreatePage(create_page::CreatePage { page: base.pages[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeletePage
