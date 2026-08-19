//! 🔧 `change-service-class` payload — changes the En1995 document's `service_class` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeServiceClass
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeServiceClass {
    pub new_service_class: String,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeServiceClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "service-class", kind: "change-service-class", record: "ChangedServiceClass" };

    async fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::change_service_class::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_service_class::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change service class to {:?}", self.new_service_class)
    }
}
//#endregion 🔖️ChangeServiceClass
