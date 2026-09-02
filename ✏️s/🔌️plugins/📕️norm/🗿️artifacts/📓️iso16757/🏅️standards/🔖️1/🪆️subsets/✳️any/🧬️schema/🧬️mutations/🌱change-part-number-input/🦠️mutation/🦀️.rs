//! 🔢️ `change-part-number-input` — upserts one scripted part-number input value, addressed by key.

use crate::artifacts::iso16757::{CatalogueValue, Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ChangePartNumberInput {
    pub key: String,
    pub new_value: CatalogueValue,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for ChangePartNumberInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "part-number-input", kind: "change-part-number-input", record: "ChangedPartNumberInput" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change part-number input \"{}\"", self.key)
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Payload
