//! 🏛️ `update-page-columns` — atomically sets a page's column count and gutter together.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🏛️UpdatePageColumns
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdatePageColumns {
    pub id: String,
    pub count: u32,
    pub gutter: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdatePageColumns {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "page-columns", kind: "update-page-columns", record: "UpdatedPageColumns" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_page_columns(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_update_page_columns(self, base)
    }
    async fn label(&self) -> String {
        format!("Update page \"{}\" columns", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🏛️UpdatePageColumns


//#region 🏛️UpdatePageColumns
pub async fn diff_update_page_columns(payload: &UpdatePageColumns, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.columns.count == payload.count && page.columns.gutter == payload.gutter {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has those columns.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { columns_count: Some(payload.count), columns_gutter: Some(payload.gutter), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🏛️UpdatePageColumns


//#region 🏛️UpdatePageColumns
pub async fn inverse_update_page_columns(payload: &UpdatePageColumns, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::UpdatePageColumns(UpdatePageColumns { id: payload.id.clone(), count: page.columns.count, gutter: page.columns.gutter })],
        None => Vec::new(),
    }
}
//#endregion 🏛️UpdatePageColumns
