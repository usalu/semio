//! 📐 `update-page-margins` — atomically sets a page's four margin fields together (a facet that's
//! never meaningfully edited one field at a time — a margins dialog writes all four at once).

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 📐UpdatePageMargins
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePageMargins {
    pub id: String,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdatePageMargins {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "page-margins", kind: "update-page-margins", record: "UpdatedPageMargins" };
    fn diff(&self, base: &LayoutSnapshot) -> LayoutDiff {
        super::diff::diff_update_page_margins(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_update_page_margins(self, base)
    }
    fn label(&self) -> String {
        format!("Update page \"{}\" margins", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 📐UpdatePageMargins
