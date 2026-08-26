//! 🎚️ `change-airtightness-class` — sets the DIN 4108 `airtightness_class` scalar.

use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeAirtightnessClass {
    pub new_airtightness_class: String,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeAirtightnessClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "airtightness-class", kind: "change-airtightness-class", record: "ChangedAirtightnessClass" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change airtightness class to \"{}\"", self.new_airtightness_class)
    }
}
//#endregion 🔖️Payload
