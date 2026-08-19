//! 🏷️ `change-category` — sets the DIN 4108 `category` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeCategory {
    pub new_category: String,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeCategory {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "category", kind: "change-category", record: "ChangedCategory" };

    async fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change category to \"{}\"", self.new_category)
    }
}
//#endregion 🔖️Payload
