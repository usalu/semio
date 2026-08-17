//! 🗑️ `delete-page` — removes a {@link Page} by id; inverse recreates it via `create-page`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🗑️DeletePage
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletePage {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeletePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "page", kind: "delete-page", record: "DeletedPage" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_delete_page(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_delete_page(self, base)
    }
    fn label(&self) -> String {
        format!("Delete page \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeletePage
