//! 📋 `change-declared-application-class` — sets the DIN 4108 `declared_application_class` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeDeclaredApplicationClass {
    pub new_declared_application_class: String,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeDeclaredApplicationClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "declared-application-class", kind: "change-declared-application-class", record: "ChangedDeclaredApplicationClass" };

    async fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change declared application class to \"{}\"", self.new_declared_application_class)
    }
}
//#endregion 🔖️Payload
