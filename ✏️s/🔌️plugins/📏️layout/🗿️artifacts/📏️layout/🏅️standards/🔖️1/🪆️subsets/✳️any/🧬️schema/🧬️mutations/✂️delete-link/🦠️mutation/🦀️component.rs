//! 🗑️ `delete-link` — removes an {@link ImageLink} by id; inverse recreates it via `create-link`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🗑️DeleteLink
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteLink {
    pub id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteLink {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "link", kind: "delete-link", record: "DeletedLink" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_delete_link(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_delete_link(self, base)
    }
    fn label(&self) -> String {
        format!("Delete link \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🗑️DeleteLink
