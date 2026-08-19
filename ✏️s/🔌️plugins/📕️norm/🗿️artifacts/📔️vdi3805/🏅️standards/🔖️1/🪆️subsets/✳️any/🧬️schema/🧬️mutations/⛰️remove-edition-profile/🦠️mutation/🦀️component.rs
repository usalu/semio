//! ➖️ `remove-edition-profile` — clears one sheet's edition-profile override (reverting it to the
//! evaluator's default of `EditionProfileChoice::Current`).

use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveEditionProfile {
    pub sheet: String,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for RemoveEditionProfile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "edition-profile", kind: "remove-edition-profile", record: "RemovedEditionProfile" };

    async fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove edition profile override for sheet {}", self.sheet)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.sheet.clone()]
    }
}
//#endregion 🔖️Payload
