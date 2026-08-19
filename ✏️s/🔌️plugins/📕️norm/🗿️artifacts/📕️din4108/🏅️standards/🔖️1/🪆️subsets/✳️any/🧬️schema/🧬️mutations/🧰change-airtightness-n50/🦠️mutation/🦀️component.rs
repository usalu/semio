//! 💨 `change-airtightness-n50` — sets the DIN 4108 `airtightness_n50` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAirtightnessN50 {
    pub new_airtightness_n50: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeAirtightnessN50 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "airtightness-n50", kind: "change-airtightness-n50", record: "ChangedAirtightnessN50" };

    async fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change airtightness n50 to {}", self.new_airtightness_n50)
    }
}
//#endregion 🔖️Payload
