//! 🗂️ `change-application-type` — sets the DIN 4108 `application_type` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeApplicationType {
    pub new_application_type: String,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeApplicationType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "application-type", kind: "change-application-type", record: "ChangedApplicationType" };

    async fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change application type to \"{}\"", self.new_application_type)
    }
}
//#endregion 🔖️Payload
