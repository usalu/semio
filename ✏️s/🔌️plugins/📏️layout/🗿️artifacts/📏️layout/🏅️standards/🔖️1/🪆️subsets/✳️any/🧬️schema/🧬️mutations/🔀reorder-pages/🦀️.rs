//! 🔀 `reorder-pages` — repositions a page within the display-ordered `pages` list (document page
//! sequence, unlike `stories`/`links` which have no display order).


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::LayoutPagesDelta;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔀ReorderPages
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ReorderPages {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ReorderPages {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "pages", kind: "reorder-pages", record: "ReorderedPages" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_reorder_pages(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_reorder_pages(self, base)
    }
    async fn label(&self) -> String {
        format!("Reorder page \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀ReorderPages


//#region 🔀ReorderPages
pub async fn diff_reorder_pages(payload: &ReorderPages, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !base.pages.iter().any(|page| page.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let current: Vec<String> = base.pages.iter().map(|page| page.id.clone()).collect();
    let mut ids = current.clone();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    if ids == current {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" is already at the requested position.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() })
}
//#endregion 🔀ReorderPages


//#region 🔀ReorderPages
pub async fn inverse_reorder_pages(payload: &ReorderPages, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().position(|page| page.id == payload.id) {
        Some(original_index) => vec![LayoutMutation::ReorderPages(ReorderPages { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }
}
//#endregion 🔀ReorderPages
