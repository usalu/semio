//! ↔️ `change-page-width` — sets a page's `width` scalar.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ↔️ChangePageWidth
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePageWidth {
    pub id: String,
    pub new_width: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangePageWidth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "page-width", kind: "change-page-width", record: "ChangedPageWidth" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_change_page_width(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_page_width(self, base)
    }
    async fn label(&self) -> String {
        format!("Change page \"{}\" width", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ↔️ChangePageWidth
