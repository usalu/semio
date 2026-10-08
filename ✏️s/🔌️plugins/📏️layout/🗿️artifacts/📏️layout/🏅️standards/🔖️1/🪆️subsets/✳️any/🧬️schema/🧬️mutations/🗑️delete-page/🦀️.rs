//! 🗑️ `delete-page` — removes a {@link Page} by id; inverse recreates it via `create-page`.

use crate::mutations::{create_page, LayoutMutation};
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPageRemoval};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗑️DeletePage
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeletePage {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeletePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "page", kind: "delete-page", record: "DeletedPage" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_page(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_delete_page(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete page \"{}\"", self.id), &format!("Seite \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeletePage

//#region 🗑️DeletePage
pub fn diff_delete_page(payload: &DeletePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.pages.iter().position(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec![LayoutPageRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🗑️DeletePage

//#region 🗑️DeletePage
pub fn inverse_delete_page(payload: &DeletePage, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.pages.iter().position(|page| page.id == payload.id) {
        Some(index) => vec![LayoutMutation::CreatePage(create_page::CreatePage { page: base.pages[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🗑️DeletePage
