//! 🏷️ `rename-catalogue` — renames the catalogue's identity field (metadata preferred name).

use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameCatalogue {
    pub new_name: String,
}

impl protocol::MutationKind<Iso16757Snapshot, Iso16757Mutation> for RenameCatalogue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "catalogue", kind: "rename-catalogue", record: "RenamedCatalogue" };

    fn diff(&self, base: &Iso16757Snapshot) -> protocol::MutationOutcome<<Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename catalogue to \"{}\"", self.new_name)
    }
}
//#endregion 🔖️Payload
