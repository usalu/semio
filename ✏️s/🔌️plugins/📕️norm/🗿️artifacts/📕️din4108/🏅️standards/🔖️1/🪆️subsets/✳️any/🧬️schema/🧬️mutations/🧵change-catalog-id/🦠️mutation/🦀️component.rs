//! 📇 `change-catalog-id` — sets the DIN 4108 `catalog_id` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeCatalogId {
    pub new_catalog_id: String,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeCatalogId {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "catalog-id", kind: "change-catalog-id", record: "ChangedCatalogId" };

    async fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change catalog id to \"{}\"", self.new_catalog_id)
    }
}
//#endregion 🔖️Payload
