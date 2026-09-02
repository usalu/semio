//! 🏭️ `rename-manufacturer` — renames the catalogue's manufacturer identity field.

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct RenameManufacturer {
    pub new_name: String,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for RenameManufacturer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "manufacturer", kind: "rename-manufacturer", record: "RenamedManufacturer" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename manufacturer to \"{}\"", self.new_name)
    }
}
//#endregion 🔖️Payload
