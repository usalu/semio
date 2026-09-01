//! ✏️ `rename-layout` — changes the document's identity `name` field.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use crate::artifacts::layout::mutations::LayoutMutation;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ✏️RenameLayout
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameLayout {
    pub new_name: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for RenameLayout {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "layout", kind: "rename-layout", record: "RenamedLayout" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_rename_layout(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_rename_layout(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename document to \"{}\"", self.new_name)
    }
}
//#endregion ✏️RenameLayout


//#region ✏️RenameLayout
pub async fn diff_rename_layout(payload: &RenameLayout, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if base.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Layout already has that name.");
    }
    protocol::MutationOutcome::new(LayoutDiff { name: Some(payload.new_name.clone()), ..Default::default() })
}
//#endregion ✏️RenameLayout


//#region ✏️RenameLayout
pub async fn inverse_rename_layout(_payload: &RenameLayout, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::RenameLayout(RenameLayout { new_name: base.name.clone() })]
}
//#endregion ✏️RenameLayout
