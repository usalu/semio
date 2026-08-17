//! 🏛️ `update-page-columns` — atomically sets a page's column count and gutter together.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🏛️UpdatePageColumns
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePageColumns {
    pub id: String,
    pub count: u32,
    pub gutter: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdatePageColumns {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "page-columns", kind: "update-page-columns", record: "UpdatedPageColumns" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_update_page_columns(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_update_page_columns(self, base)
    }
    fn label(&self) -> String {
        format!("Update page \"{}\" columns", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🏛️UpdatePageColumns
