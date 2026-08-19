//! ✏️ `rename-layout` — changes the document's identity `name` field.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ✏️RenameLayout
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameLayout {
    pub new_name: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for RenameLayout {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "layout", kind: "rename-layout", record: "RenamedLayout" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_rename_layout(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_rename_layout(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename document to \"{}\"", self.new_name)
    }
}
//#endregion ✏️RenameLayout
