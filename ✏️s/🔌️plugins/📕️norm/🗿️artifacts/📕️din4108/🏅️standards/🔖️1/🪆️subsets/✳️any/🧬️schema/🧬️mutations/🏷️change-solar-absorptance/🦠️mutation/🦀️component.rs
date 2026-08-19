//! ☀️ `change-solar-absorptance` — sets the DIN 4108 `solar_absorptance` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSolarAbsorptance {
    pub new_solar_absorptance: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeSolarAbsorptance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "solar-absorptance", kind: "change-solar-absorptance", record: "ChangedSolarAbsorptance" };

    async fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change solar absorptance to {}", self.new_solar_absorptance)
    }
}
//#endregion 🔖️Payload
