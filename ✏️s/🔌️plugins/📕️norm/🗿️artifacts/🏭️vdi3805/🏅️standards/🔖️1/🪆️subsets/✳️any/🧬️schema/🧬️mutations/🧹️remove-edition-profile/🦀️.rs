//! ➖️ `remove-edition-profile` — clears one sheet's edition-profile override (reverting it to the
//! evaluator's default of `EditionProfileChoice::Current`).


use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use crate::artifacts::vdi3805::mutations::change_edition_profile;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveEditionProfile {
    pub sheet: String,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for RemoveEditionProfile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "edition-profile", kind: "remove-edition-profile", record: "RemovedEditionProfile" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove edition profile override for sheet {}", self.sheet)
    }
    fn target(&self) -> Vec<String> {
        vec![self.sheet.clone()]
    }
}
//#endregion 🔖️Payload
