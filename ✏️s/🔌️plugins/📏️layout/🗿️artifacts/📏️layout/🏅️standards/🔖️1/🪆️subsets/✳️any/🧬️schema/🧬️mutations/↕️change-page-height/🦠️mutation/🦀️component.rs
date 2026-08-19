//! ↕️ `change-page-height` — sets a page's `height` scalar.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ↕️ChangePageHeight
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangePageHeight {
    pub id: String,
    pub new_height: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangePageHeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "page-height", kind: "change-page-height", record: "ChangedPageHeight" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_change_page_height(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_page_height(self, base)
    }
    async fn label(&self) -> String {
        format!("Change page \"{}\" height", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ↕️ChangePageHeight
