//! 📐 `update-page-margins` — atomically sets a page's four margin fields together (a facet that's
//! never meaningfully edited one field at a time — a margins dialog writes all four at once).


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📐UpdatePageMargins
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdatePageMargins {
    pub id: String,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdatePageMargins {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "page-margins", kind: "update-page-margins", record: "UpdatedPageMargins" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_page_margins(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_update_page_margins(self, base)
    }
    async fn label(&self) -> String {
        format!("Update page \"{}\" margins", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 📐UpdatePageMargins


//#region 📐UpdatePageMargins
pub async fn diff_update_page_margins(payload: &UpdatePageMargins, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.margins.top == payload.top && page.margins.right == payload.right && page.margins.bottom == payload.bottom && page.margins.left == payload.left {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Page \"{}\" already has those margins.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.id.clone(),
                patch: PagePatch { margin_top: Some(payload.top), margin_right: Some(payload.right), margin_bottom: Some(payload.bottom), margin_left: Some(payload.left), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 📐UpdatePageMargins


//#region 📐UpdatePageMargins
pub async fn inverse_update_page_margins(payload: &UpdatePageMargins, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::UpdatePageMargins(UpdatePageMargins { id: payload.id.clone(), top: page.margins.top, right: page.margins.right, bottom: page.margins.bottom, left: page.margins.left })],
        None => Vec::new(),
    }
}
//#endregion 📐UpdatePageMargins
