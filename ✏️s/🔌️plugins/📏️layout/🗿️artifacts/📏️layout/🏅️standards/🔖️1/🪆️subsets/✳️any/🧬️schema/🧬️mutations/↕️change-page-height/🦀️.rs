//! ↕️ `change-page-height` — sets a page's `height` scalar.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ↕️ChangePageHeight
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangePageHeight {
    pub id: String,
    pub new_height: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangePageHeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "page-height", kind: "change-page-height", record: "ChangedPageHeight" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_page_height(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_page_height(self, base)
    }
    async fn label(&self) -> String {
        format!("Change page \"{}\" height", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ↕️ChangePageHeight


//#region ↕️ChangePageHeight
pub async fn diff_change_page_height(payload: &ChangePageHeight, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.height == payload.new_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has height {}.", payload.id, payload.new_height));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { height: Some(payload.new_height), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion ↕️ChangePageHeight


//#region ↕️ChangePageHeight
pub async fn inverse_change_page_height(payload: &ChangePageHeight, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::ChangePageHeight(ChangePageHeight { id: payload.id.clone(), new_height: page.height })],
        None => Vec::new(),
    }
}
//#endregion ↕️ChangePageHeight
