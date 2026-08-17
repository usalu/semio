//! 📐 `change-envelope-area-m2` — sets the DIN 4108 `envelope_area_m2` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeEnvelopeAreaM2 {
    pub new_envelope_area_m2: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeEnvelopeAreaM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "envelope-area-m2", kind: "change-envelope-area-m2", record: "ChangedEnvelopeAreaM2" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change envelope area m2 to {}", self.new_envelope_area_m2)
    }
}
//#endregion 🔖️Payload
