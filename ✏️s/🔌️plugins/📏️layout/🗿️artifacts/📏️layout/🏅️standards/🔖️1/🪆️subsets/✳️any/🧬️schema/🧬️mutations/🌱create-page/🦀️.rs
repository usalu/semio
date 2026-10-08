//! 🌱️ `create-page` — brings a new {@link Page} into existence in the id-keyed `pages` collection.

use crate::mutations::{delete_page, LayoutMutation};
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPageInsertion};
use crate::{LayoutDiff, LayoutSnapshot, Page};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🌱️CreatePage
/// 🌱️ `index` is the zero-based insertion position among the pages; `None` or past the end appends.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreatePage {
    pub page: Page,
    pub index: Option<usize>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreatePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "page", kind: "create-page", record: "CreatedPage" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_page(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_create_page(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create page \"{}\"", self.page.name), &format!("Seite \"{}\" erstellen", self.page.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page.id.clone()]
    }
}
//#endregion 🌱️CreatePage

//#region 🌱️CreatePage
pub fn diff_create_page(payload: &CreatePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.pages.iter().any(|page| page.id == payload.page.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A page with id \"{}\" already exists.", payload.page.id), [payload.page.id.clone()]);
    }
    if payload.index.is_some_and(|at| at > base.pages.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.pages.len()), [&payload.page.id.to_string()]);
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { inserted: vec![LayoutPageInsertion { index: payload.index.unwrap_or(base.pages.len()), row: payload.page.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🌱️CreatePage

//#region 🌱️CreatePage
pub fn inverse_create_page(payload: &CreatePage, _base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![LayoutMutation::DeletePage(delete_page::DeletePage { id: payload.page.id.clone() })]

    })())
}
//#endregion 🌱️CreatePage
