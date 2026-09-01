//! 🏷️ `rename-product` — renames a catalogue product's display title, addressed by article number.


use crate::artifacts::vdi3805::{LocalizedText, Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameProduct {
    pub id: String,
    pub new_title: Vec<LocalizedText>,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for RenameProduct {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "product", kind: "rename-product", record: "RenamedProduct" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename product \"{}\" to \"{}\"", self.id, crate::artifacts::vdi3805::text_in(&self.new_title, "en"))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
