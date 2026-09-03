//! 🌱️ `create-page` — brings a new {@link Page} into existence in the id-keyed `pages` collection.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, Page};
use crate::artifacts::layout::mutations::{LayoutMutation, delete_page};
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🌱️CreatePage
/// 🌱️ `index` is descriptive of authoring intent (the append-only `LayoutPagesDelta` apply always
/// pushes at the end, matching the pre-migration generic append behavior).
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct CreatePage {
    pub page: Page,
    pub index: Option<usize>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreatePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "page", kind: "create-page", record: "CreatedPage" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_page(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_create_page(self, base)
    }
    async fn label(&self) -> String {
        format!("Create page \"{}\"", self.page.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page.id.clone()]
    }
}
//#endregion 🌱️CreatePage


//#region 🌱️CreatePage
pub async fn diff_create_page(payload: &CreatePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.pages.iter().any(|page| page.id == payload.page.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A page with id \"{}\" already exists.", payload.page.id), [payload.page.id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { added: vec![payload.page.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🌱️CreatePage


//#region 🌱️CreatePage
pub async fn inverse_create_page(payload: &CreatePage, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeletePage(delete_page::DeletePage { id: payload.page.id.clone() })]
}
//#endregion 🌱️CreatePage
