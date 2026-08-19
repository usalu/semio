//! 💦 `change-moisture-mu-interior` — sets the DIN 4108 `moisture_mu_interior` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeMoistureMuInterior {
    pub new_moisture_mu_interior: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeMoistureMuInterior {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "moisture-mu-interior", kind: "change-moisture-mu-interior", record: "ChangedMoistureMuInterior" };

    async fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change moisture mu interior to {}", self.new_moisture_mu_interior)
    }
}
//#endregion 🔖️Payload
