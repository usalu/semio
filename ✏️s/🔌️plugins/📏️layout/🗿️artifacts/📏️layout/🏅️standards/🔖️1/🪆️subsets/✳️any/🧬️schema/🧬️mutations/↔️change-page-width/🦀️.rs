//! ↔️ `change-page-width` — sets a page's `width` scalar.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::{LayoutDiff, LayoutSnapshot, PagePatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ↔️ChangePageWidth
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangePageWidth {
    pub id: String,
    pub new_width: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangePageWidth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "page-width", kind: "change-page-width", record: "ChangedPageWidth" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_page_width(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_page_width(self, base)
    }
    fn label(&self) -> String {
        format!("Change page \"{}\" width", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ↔️ChangePageWidth

//#region ↔️ChangePageWidth
pub fn diff_change_page_width(payload: &ChangePageWidth, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.width == payload.new_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has width {}.", payload.id, payload.new_width));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { width: Some(payload.new_width), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion ↔️ChangePageWidth

//#region ↔️ChangePageWidth
pub fn inverse_change_page_width(payload: &ChangePageWidth, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::ChangePageWidth(ChangePageWidth { id: payload.id.clone(), new_width: page.width })],
        None => Vec::new(),
    }
}
//#endregion ↔️ChangePageWidth
